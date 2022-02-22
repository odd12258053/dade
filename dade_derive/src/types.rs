use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DataStruct, Fields, GenericArgument, Ident, PathArguments, Type, Visibility};

use crate::fields::ModelField;
use crate::terms::{Condition, DefaultTerm, ToSchema, ToToken};

enum ModelType {
    Null,
    Number,
    String,
    Bool,
    Optional(Box<ModelType>),
    Array,
    Object,
}

const NUMBER_TYPES: [&str; 14] = [
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64",
];

impl ModelType {
    fn new(ty: &Type) -> Self {
        match ty {
            Type::Path(type_path) => {
                let type_token = type_path.to_token_stream().to_string();
                if NUMBER_TYPES.iter().any(|&s| s == type_token) {
                    ModelType::Number
                } else if type_token == "String" {
                    ModelType::String
                } else if type_token == "bool" {
                    ModelType::Bool
                } else {
                    let segment = type_path.path.segments.iter().next().unwrap();
                    let ident = &segment.ident;
                    if ident == "Option" {
                        ModelType::Optional(Box::new({
                            match &segment.arguments {
                                PathArguments::AngleBracketed(angle_bracketed) => {
                                    if angle_bracketed.args.is_empty()
                                        || angle_bracketed.args.len() > 1
                                    {
                                        panic!("Invalid type")
                                    }
                                    match angle_bracketed.args.first().unwrap() {
                                        GenericArgument::Type(inner_type) => {
                                            ModelType::new(inner_type)
                                        }
                                        _ => {
                                            panic!("Invalid type")
                                        }
                                    }
                                }
                                _ => {
                                    panic!("Invalid type")
                                }
                            }
                        }))
                    } else if ident == "Vec" {
                        ModelType::Array
                    } else {
                        ModelType::Object
                    }
                }
            }
            Type::Tuple(type_tuple) => {
                if type_tuple.to_token_stream().to_string() == "()" {
                    ModelType::Null
                } else {
                    panic!("Invalid type")
                }
            }
            _ => panic!("Invalid type"),
        }
    }
}

fn handle_null_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &str,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    let default_val = if let Some(DefaultTerm::Ident(term)) = &model_field.default {
        let val = &term.value;
        if val == "null" {
            conds.push(quote! {
                "default".to_string(), dade::JsonValue::Null
            });
            quote! { () }
        } else {
            panic!("Support default condition is only `null`")
        }
    } else {
        quote! { () }
    };
    statements.push(quote! {
        let #variable: #variable_type = match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val)?,
            None => #default_val,
        };
    });
    if !model_field.conditions.is_empty() {
        panic!("Support condition is alias, default and validate")
    }
}

fn handle_number_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &str,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    let default_val = if let Some(DefaultTerm::Lit(term)) = &model_field.default {
        let val = &term.value;
        conds.push(quote! {
            "default".to_string(), dade::JsonValue::Number(dade::Number::from(#val))
        });
        quote! { #val }
    } else {
        let msg = format!("not found key, {}", variable_key);
        quote! {
            return Err(dade::Error::new_validate_err(#msg))
        }
    };
    statements.push(quote! {
        // TODO: set a correct error.
        let #variable: #variable_type = match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val)?,
            None => #default_val,
        };
    });
    let mut terms = Vec::new();
    for cond in model_field.conditions.iter() {
        match cond {
            Condition::Gt(term) => {
                terms.push(term.to_token(variable));
                conds.push(term.to_schema());
            }
            Condition::Ge(term) => {
                terms.push(term.to_token(variable));
                conds.push(term.to_schema());
            }
            Condition::Lt(term) => {
                terms.push(term.to_token(variable));
                conds.push(term.to_schema());
            }
            Condition::Le(term) => {
                terms.push(term.to_token(variable));
                conds.push(term.to_schema());
            }
            _ => {
                panic!("Support condition is gt, ge, lt, le, alias, default and validate")
            }
        }
    }
    if !terms.is_empty() {
        statements.push(quote! {
            if !( #(#terms)&&* ) {
                return Err(dade::Error::new_validate_err("invalid number"))
            }
        })
    }
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        statements.push(quote! {
            let #variable: #variable_type = #fn_name(#variable)?;
        });
    }
}

fn handle_string_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &str,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    let default_val = if let Some(DefaultTerm::Lit(term)) = &model_field.default {
        let val = &term.value;
        conds.push(quote! {
            "default".to_string(), dade::JsonValue::String(#val.to_string())
        });
        quote! { #val.to_string() }
    } else {
        let msg = format!("not found key, {}", variable_key);
        quote! {
            return Err(dade::Error::new_validate_err(#msg))
        }
    };
    statements.push(quote! {
        let #variable: #variable_type = match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val)?,
            None => #default_val,
        };
    });

    let mut terms = Vec::new();
    for cond in model_field.conditions.iter() {
        match cond {
            Condition::MinLength(term) => {
                terms.push(term.to_token(variable));
                conds.push(term.to_schema());
            }
            Condition::MaxLength(term) => {
                terms.push(term.to_token(variable));
                conds.push(term.to_schema());
            }
            _ => panic!("Support condition is min_length, max_length, alias, default and validate"),
        }
    }
    if !terms.is_empty() {
        statements.push(quote! {
            if !( #(#terms)&&* ) {
                return Err(dade::Error::new_validate_err("invalid string"))
            }
        });
    }
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        statements.push(quote! {
            let #variable: #variable_type = #fn_name(#variable)?;
        });
    }
}

fn handle_bool_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &str,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    let default_val = if let Some(DefaultTerm::Lit(term)) = &model_field.default {
        let val = &term.value;
        conds.push(quote! {
            "default".to_string(), dade::JsonValue::Bool(#val)
        });
        quote! { #val }
    } else {
        let msg = format!("not found key, {}", variable_key);
        quote! {
            return Err(dade::Error::new_validate_err(#msg))
        }
    };
    statements.push(quote! {
        let #variable: #variable_type = match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val)?,
            None => #default_val,
        };
    });
    if !model_field.conditions.is_empty() {
        panic!("Support condition is alias, default and validate")
    }
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        statements.push(quote! {
            let #variable: #variable_type = #fn_name(#variable)?;
        });
    }
}

fn handle_optional_type(
    inner_type: &ModelType,
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &str,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    let default_val = if let Some(term) = &model_field.default {
        match inner_type {
            ModelType::Null => panic!("invalid type. You only use `()`."),
            ModelType::Number => match term {
                DefaultTerm::Ident(term) if term.value == "null" => {
                    conds.push(quote! {
                        "default".to_string(), dade::JsonValue::Null
                    });
                    quote! { None }
                }
                DefaultTerm::Lit(term) => {
                    let val = &term.value;
                    conds.push(quote! {
                        "default".to_string(), dade::JsonValue::Number(dade::Number::from(#val))
                    });
                    quote! { #val }
                }
                _ => panic!("Support default condition is `null` or Number"),
            },
            ModelType::String => match term {
                DefaultTerm::Ident(term) if term.value == "null" => {
                    conds.push(quote! {
                        "default".to_string(), dade::JsonValue::Null
                    });
                    quote! { None }
                }
                DefaultTerm::Lit(term) => {
                    let val = &term.value;
                    conds.push(quote! {
                        "default".to_string(), dade::JsonValue::String(#val.to_string())
                    });
                    quote! { #val.to_string() }
                }
                _ => panic!("Support default condition is `null` or String"),
            },
            ModelType::Bool => match term {
                DefaultTerm::Ident(term) if term.value == "null" => {
                    conds.push(quote! {
                        "default".to_string(), dade::JsonValue::Null
                    });
                    quote! { None }
                }
                DefaultTerm::Lit(term) => {
                    let val = &term.value;
                    conds.push(quote! {
                        "default".to_string(), dade::JsonValue::Bool(#val)
                    });
                    quote! { #val }
                }
                _ => panic!("Support default condition is `null`, `false`, `true`"),
            },
            ModelType::Optional(_) => panic!("invalid type"),
            ModelType::Array => {
                panic!("Support default condition is only `null`")
            }
            ModelType::Object => {
                panic!("Support default condition is only `null`")
            }
        }
    } else {
        quote! { None }
    };

    statements.push(quote! {
        let #variable: #variable_type = match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val)?,
            None => #default_val,
        };
    });
    if !model_field.conditions.is_empty() {
        let mut terms = Vec::new();
        let inner_type_name = match inner_type {
            ModelType::Number => {
                for cond in model_field.conditions.iter() {
                    match cond {
                        Condition::Gt(term) => {
                            terms.push(term.to_token(variable));
                            conds.push(term.to_schema());
                        }
                        Condition::Ge(term) => {
                            terms.push(term.to_token(variable));
                            conds.push(term.to_schema());
                        }
                        Condition::Lt(term) => {
                            terms.push(term.to_token(variable));
                            conds.push(term.to_schema());
                        }
                        Condition::Le(term) => {
                            terms.push(term.to_token(variable));
                            conds.push(term.to_schema());
                        }
                        _ => {
                            panic!(
                                "Support condition is gt, ge, lt, le, alias, default and validate"
                            )
                        }
                    }
                }
                "number"
            }
            ModelType::String => {
                for cond in model_field.conditions.iter() {
                    match cond {
                        Condition::MinLength(term) => {
                            terms.push(term.to_token(variable));
                            conds.push(term.to_schema());
                        },
                        Condition::MaxLength(term) => {
                            terms.push(term.to_token(variable));
                            conds.push(term.to_schema());
                        },
                        _ => panic!("Support condition is min_length, max_length, alias, default and validate"),
                    }
                }
                "string"
            }
            ModelType::Array => {
                for cond in model_field.conditions.iter() {
                    match cond {
                        Condition::MinItems(term) => {
                            terms.push(term.to_token(variable));
                            conds.push(term.to_schema());
                        }
                        Condition::MaxItems(term) => {
                            terms.push(term.to_token(variable));
                            conds.push(term.to_schema());
                        }
                        _ => {
                            panic!("Support condition is min_items, max_items, alias and validate")
                        }
                    }
                }
                "array"
            }
            _ => {
                panic!("Support condition is alias, default and validate")
            }
        };
        if !terms.is_empty() {
            let err_msg = format!("invalid {}", inner_type_name);
            statements.push(quote! {
                if let Some(ref #variable) = #variable {
                    if !( #(#terms)&&* ) {
                        return Err(dade::Error::new_validate_err(#err_msg))
                    }
                }
            });
        }
    }

    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        statements.push(quote! {
            let #variable: #variable_type = #fn_name(#variable)?;
        });
    }
}

fn handle_array_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &str,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    if model_field.default.is_some() {
        panic!("Support condition is min_items, max_items, alias and validate")
    }
    let msg = format!("not found key, {}", variable_key);
    statements.push(quote! {
        let #variable: #variable_type = match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val)?,
            None => return Err(dade::Error::new_validate_err(#msg)),
        };
    });
    let mut terms = Vec::new();
    for cond in model_field.conditions.iter() {
        match cond {
            Condition::MinItems(term) => {
                terms.push(term.to_token(variable));
                conds.push(term.to_schema());
            }
            Condition::MaxItems(term) => {
                terms.push(term.to_token(variable));
                conds.push(term.to_schema());
            }
            _ => panic!("Support condition is min_items, max_items, alias and validate"),
        }
    }
    if !terms.is_empty() {
        statements.push(quote! {
            if !( #(#terms)&&* ) {
                return Err(dade::Error::new_validate_err("invalid array"))
            }
        });
    }
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        statements.push(quote! {
            let #variable: #variable_type = #fn_name(#variable)?;
        });
    }
}

fn handle_object_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &str,
    statements: &mut Vec<TokenStream>,
    _conds: &mut Vec<TokenStream>,
) {
    if model_field.default.is_some() {
        panic!("Support condition is alias and validate")
    }
    let msg = format!("not found key, {}", variable_key);
    statements.push(quote! {
        let #variable: #variable_type = match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val)?,
            None => return Err(dade::Error::new_validate_err(#msg)),
        };
    });
    if !model_field.conditions.is_empty() {
        panic!("Support condition is alias, default and validate")
    }
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        statements.push(quote! {
            let #variable: #variable_type = #fn_name(#variable)?;
        });
    }
}

pub(crate) fn handle_struct(ident: Ident, vis: Visibility, data: DataStruct) -> TokenStream {
    match data.fields {
        Fields::Named(fields_named) => {
            let mut fields = Vec::new();
            let mut maps = Vec::new();
            let mut keys = Vec::new();
            let mut statements = Vec::new();
            let mut schemas = Vec::new();
            let mut required = Vec::new();

            for field in fields_named.named.iter() {
                let (attrs, model_field) = {
                    let mut bag = Vec::new();
                    let mut model_field = ModelField::default();
                    for attr in field.attrs.iter() {
                        if attr.path.get_ident().unwrap() == "field" {
                            if !attr.tokens.is_empty() {
                                model_field = attr.parse_args().unwrap();
                            }
                        } else {
                            bag.push(attr)
                        }
                    }
                    (quote! {#(#bag)*}, model_field)
                };
                let variable: &Ident = field.ident.as_ref().unwrap();
                let variable_vis = &field.vis;
                let variable_key = if let Some(alias) = &model_field.alias {
                    alias.value.value()
                } else {
                    format!("{}", variable)
                };
                maps.push(quote! {
                    (
                        #variable_key.to_string(),
                        dade::ToJsonValue::to_json_value(&self.#variable)
                    )
                });
                keys.push(quote! {#variable});
                let ty = &field.ty;
                let mut conds: Vec<TokenStream> = Vec::from([quote! {
                    "title".to_string(),
                    dade::JsonValue::String(dade::ToTitle::to_title(#variable_key))
                }]);
                let model_type = ModelType::new(ty);
                if model_field.default.is_none() {
                    match model_type {
                        ModelType::Optional(_) => (),
                        _ => required.push(quote! { #variable_key }),
                    }
                }
                match model_type {
                    ModelType::Null => handle_null_type(
                        &model_field,
                        variable,
                        ty,
                        &variable_key,
                        &mut statements,
                        &mut conds,
                    ),
                    ModelType::Number => handle_number_type(
                        &model_field,
                        variable,
                        ty,
                        &variable_key,
                        &mut statements,
                        &mut conds,
                    ),
                    ModelType::String => handle_string_type(
                        &model_field,
                        variable,
                        ty,
                        &variable_key,
                        &mut statements,
                        &mut conds,
                    ),
                    ModelType::Bool => handle_bool_type(
                        &model_field,
                        variable,
                        ty,
                        &variable_key,
                        &mut statements,
                        &mut conds,
                    ),
                    ModelType::Optional(inner_type) => handle_optional_type(
                        &inner_type,
                        &model_field,
                        variable,
                        ty,
                        &variable_key,
                        &mut statements,
                        &mut conds,
                    ),
                    ModelType::Array => handle_array_type(
                        &model_field,
                        variable,
                        ty,
                        &variable_key,
                        &mut statements,
                        &mut conds,
                    ),
                    ModelType::Object => handle_object_type(
                        &model_field,
                        variable,
                        ty,
                        &variable_key,
                        &mut statements,
                        &mut conds,
                    ),
                }
                schemas.push(quote! {
                    (
                        #variable_key.to_string(),
                        {
                            let mut s = <#ty as dade::RegisterSchema>::register_schema(defs);
                            if let dade::JsonValue::Object(ref mut dict) = s {
                                #(dict.insert(#conds));*;
                            }
                            s
                        }
                    )
                });
                let colon_token = field.colon_token;
                fields.push(quote! {#attrs #variable_vis #variable #colon_token #ty});
            }

            let name = format!("{}", ident);
            let data_type = data.struct_token;
            let def_name = format!("#/definitions/{}", ident);
            quote! {
                #vis #data_type #ident { #(#fields),* }
                impl dade::ToJsonValue for #ident {
                    fn to_json_value(&self) -> dade::JsonValue {
                        dade::JsonValue::Object(
                            std::collections::BTreeMap::from( [#(#maps),*] )
                        )
                    }
                }
                impl dade::FromJsonValue for #ident {
                    fn from_json_value(value: &dade::JsonValue) -> dade::Result<Self> {
                        match value {
                            dade::JsonValue::Object(dict) => {
                                #(#statements)*
                                Ok(#ident { #(#keys),* })
                            }
                            _ => Err(dade::Error::new_validate_err("expect `JsonValue::Object`")),
                        }
                    }
                }
                impl dade::RegisterSchema for #ident {
                    fn register_schema(defs: &mut std::collections::BTreeMap<String, dade::JsonValue>) -> dade::JsonValue {
                        if !defs.contains_key(&#name.to_string()) {
                            // Insert temporarily value.
                            defs.insert(#name.to_string(), dade::JsonValue::Null);
                            let json_value = dade::JsonValue::Object(
                                    std::collections::BTreeMap::from([
                                        (
                                            "title".to_string(),
                                            dade::JsonValue::String(dade::ToTitle::to_title(#name))
                                        ),
                                        (
                                            "type".to_string(),
                                            dade::JsonValue::String("object".to_string())
                                        ),
                                        (
                                            "properties".to_string(),
                                            dade::JsonValue::Object(
                                                std::collections::BTreeMap::from([#(#schemas),*])
                                            )
                                        ),
                                        (
                                            "required".to_string(),
                                            dade::JsonValue::Array(
                                                Vec::from([
                                                    #(dade::JsonValue::String(#required.to_string())),*
                                                ])
                                            )
                                        ),
                                    ])
                                );
                            // Swap to proper value.
                            defs.insert(#name.to_string(), json_value);
                        }
                        dade::JsonValue::Object(
                            std::collections::BTreeMap::from([
                                (
                                    "$ref".to_string(),
                                    dade::JsonValue::String(#def_name.to_string())
                                ),
                            ])
                        )
                    }
                }
            }
        }
        _ => panic!("Only support named field."),
    }
}
