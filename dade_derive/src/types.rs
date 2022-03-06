use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    Attribute, DataEnum, DataStruct, Fields, GenericArgument, Ident, PathArguments, Type,
    Visibility,
};

use crate::fields::ModelField;
use crate::terms::{Condition, DefaultTerm, ToSchema, ToValidateToken};

enum ModelType {
    Null,
    Number,
    String,
    Bool,
    Optional(Box<ModelType>),
    Array,
    Other,
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
                                        _ => panic!("Invalid type"),
                                    }
                                }
                                _ => panic!("Invalid type"),
                            }
                        }))
                    } else if ident == "Vec" {
                        ModelType::Array
                    } else {
                        ModelType::Other
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

fn make_null_condition(model_field: &ModelField) {
    if !model_field.conditions.is_empty() {
        panic!("Support condition is alias, default and validate")
    }
}

fn handle_null_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &TokenStream,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    make_null_condition(model_field);
    let default_val = match &model_field.default {
        Some(DefaultTerm::Ident(term)) => {
            if &term.value == "null" {
                conds.push(quote! { "default".to_string(), dade::JsonValue::Null });
                quote! { () }
            } else {
                panic!("Support default condition is only `null`")
            }
        }
        None => quote! { () },
        Some(DefaultTerm::Lit(_)) => panic!("Support default condition is only `null`"),
    };
    statements.push(quote! {
        let #variable: #variable_type = match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val)?,
            None => #default_val,
        };
    });
}

fn make_number_condition(
    variable: &Ident,
    model_field: &ModelField,
    stmt: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    for cond in model_field.conditions.iter() {
        match cond {
            Condition::Gt(term) => {
                stmt.push(term.to_validate_token(variable));
                conds.push(term.to_schema());
            }
            Condition::Ge(term) => {
                stmt.push(term.to_validate_token(variable));
                conds.push(term.to_schema());
            }
            Condition::Lt(term) => {
                stmt.push(term.to_validate_token(variable));
                conds.push(term.to_schema());
            }
            Condition::Le(term) => {
                stmt.push(term.to_validate_token(variable));
                conds.push(term.to_schema());
            }
            _ => panic!("Support condition is gt, ge, lt, le, alias, default and validate"),
        }
    }
}

fn handle_number_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &TokenStream,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    let mut stmt = Vec::new();
    make_number_condition(variable, model_field, &mut stmt, conds);
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        stmt.push(quote! { #fn_name });
    }
    let default_val = match &model_field.default {
        Some(DefaultTerm::Lit(term)) => {
            let val = &term.value;
            conds.push(
                quote! { "default".to_string(), dade::JsonValue::Number(dade::Number::from(#val)) },
            );
            quote! { Ok(#val) }
        }
        None => {
            let msg = format!("not found key, {}", variable_key);
            quote! { Err(dade::Error::new_validate_err(#msg)) }
        }
        Some(DefaultTerm::Ident(_)) => panic!("Support default condition is only numeric"),
    };
    statements.push(quote! {
        let #variable: #variable_type = (match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val),
            None => #default_val,
        }) #(.and_then(#stmt))*?;
    });
}

fn make_string_condition(
    variable: &Ident,
    model_field: &ModelField,
    stmt: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    for cond in model_field.conditions.iter() {
        match cond {
            Condition::MinLength(term) => {
                stmt.push(term.to_validate_token(variable));
                conds.push(term.to_schema());
            }
            Condition::MaxLength(term) => {
                stmt.push(term.to_validate_token(variable));
                conds.push(term.to_schema());
            }
            _ => panic!("Support condition is min_length, max_length, alias, default and validate"),
        }
    }
}

fn handle_string_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &TokenStream,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    let mut stmt = Vec::new();
    make_string_condition(variable, model_field, &mut stmt, conds);
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        stmt.push(quote! { #fn_name });
    }
    let default_val = match &model_field.default {
        Some(DefaultTerm::Lit(term)) => {
            let val = &term.value;
            conds.push(quote! { "default".to_string(), dade::JsonValue::String(#val.to_string()) });
            quote! { Ok(#val.to_string()) }
        }
        None => {
            let msg = format!("not found key, {}", variable_key);
            quote! { Err(dade::Error::new_validate_err(#msg)) }
        }
        Some(DefaultTerm::Ident(_)) => panic!("Support default condition is only string"),
    };
    statements.push(quote! {
        let #variable: #variable_type = (match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val),
            None => #default_val,
        }) #(.and_then(#stmt))*?;
    });
}

fn make_bool_condition(model_field: &ModelField) {
    if !model_field.conditions.is_empty() {
        panic!("Support condition is alias, default and validate")
    }
}

fn handle_bool_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &TokenStream,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    make_bool_condition(model_field);
    let default_val = match &model_field.default {
        Some(DefaultTerm::Lit(term)) => {
            let val = &term.value;
            conds.push(quote! { "default".to_string(), dade::JsonValue::Bool(#val) });
            quote! { Ok(#val) }
        }
        None => {
            let msg = format!("not found key, {}", variable_key);
            quote! { Err(dade::Error::new_validate_err(#msg)) }
        }
        Some(DefaultTerm::Ident(_)) => panic!("Support default condition is only boolean"),
    };
    let mut stmt = Vec::new();
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        stmt.push(quote! { #fn_name });
    }
    statements.push(quote! {
        let #variable: #variable_type = (match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val),
            None => #default_val,
        }) #(.and_then(#stmt))*?;
    });
}

fn handle_optional_type(
    inner_type: &ModelType,
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &TokenStream,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    let default_val = match &model_field.default {
        Some(DefaultTerm::Ident(term)) => {
            if term.value == "null" {
                conds.push(quote! {
                    "default".to_string(), dade::JsonValue::Null
                });
                quote! { None }
            } else {
                panic!("Support default condition is `null` or value for inner type.")
            }
        }
        Some(DefaultTerm::Lit(term)) => match inner_type {
            ModelType::Number => {
                let val = &term.value;
                conds.push(quote! {
                    "default".to_string(), dade::JsonValue::Number(dade::Number::from(#val))
                });
                quote! { Some(#val) }
            }
            ModelType::String => {
                let val = &term.value;
                conds.push(quote! {
                    "default".to_string(), dade::JsonValue::String(#val.to_string())
                });
                quote! { Some(#val.to_string()) }
            }
            ModelType::Bool => {
                let val = &term.value;
                conds.push(quote! {
                    "default".to_string(), dade::JsonValue::Bool(#val)
                });
                quote! { Some(#val) }
            }
            ModelType::Null => panic!("invalid type. You only use `()`."),
            ModelType::Optional(_) => panic!("invalid type. Don't support nested optional type."),
            ModelType::Array => panic!("Support default condition is only `null`"),
            ModelType::Other => panic!("Support default condition is only `null`"),
        },
        None => quote! { None },
    };

    let mut stmt = Vec::new();
    if !model_field.conditions.is_empty() {
        match inner_type {
            ModelType::Null => make_null_condition(model_field),
            ModelType::Number => make_number_condition(variable, model_field, &mut stmt, conds),
            ModelType::String => make_string_condition(variable, model_field, &mut stmt, conds),
            ModelType::Bool => make_bool_condition(model_field),
            ModelType::Optional(_) => panic!("Support condition is alias and validate"),
            ModelType::Array => make_array_condition(variable, model_field, &mut stmt, conds),
            ModelType::Other => make_other_condition(model_field),
        }
    }

    let mut cstmt = Vec::new();
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        cstmt.push(quote! { #fn_name });
    }
    statements.push(quote! {
        let #variable: #variable_type = (match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val),
            None => Ok(#default_val),
        }).and_then(|x| {
            match x {
                Some(y) => Ok(Some(Ok(y) #(.and_then(#stmt))*?)),
                None => Ok(None),
            }
        }) #(.and_then(#cstmt))*?;
    });
}

fn make_array_condition(
    variable: &Ident,
    model_field: &ModelField,
    stmt: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    for cond in model_field.conditions.iter() {
        match cond {
            Condition::MinItems(term) => {
                stmt.push(term.to_validate_token(variable));
                conds.push(term.to_schema());
            }
            Condition::MaxItems(term) => {
                stmt.push(term.to_validate_token(variable));
                conds.push(term.to_schema());
            }
            _ => panic!("Support condition is min_items, max_items, alias and validate"),
        }
    }
}

fn handle_array_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &TokenStream,
    statements: &mut Vec<TokenStream>,
    conds: &mut Vec<TokenStream>,
) {
    if model_field.default.is_some() {
        panic!("Support condition is min_items, max_items, alias and validate")
    }
    let mut stmt = Vec::new();
    make_array_condition(variable, model_field, &mut stmt, conds);
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        stmt.push(quote! { #fn_name });
    }
    let msg = format!("not found key, {}", variable_key);
    statements.push(quote! {
        let #variable: #variable_type = (match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val),
            None => Err(dade::Error::new_validate_err(#msg)),
        }) #(.and_then(#stmt))*?;
    });
}

fn make_other_condition(model_field: &ModelField) {
    if !model_field.conditions.is_empty() {
        panic!("Support condition is alias and validate")
    }
}

fn handle_other_type(
    model_field: &ModelField,
    variable: &Ident,
    variable_type: &Type,
    variable_key: &TokenStream,
    statements: &mut Vec<TokenStream>,
    _conds: &mut Vec<TokenStream>,
) {
    if model_field.default.is_some() {
        panic!("Support condition is alias and validate")
    }
    make_other_condition(model_field);
    let mut stmt = Vec::new();
    if let Some(term) = &model_field.validate {
        let fn_name = &term.value;
        stmt.push(quote! { #fn_name });
    }
    let msg = format!("not found key, {}", variable_key);
    statements.push(quote! {
        let #variable: #variable_type = (match dict.get(#variable_key) {
            Some(val) => dade::FromJsonValue::from_json_value(val),
            None => Err(dade::Error::new_validate_err(#msg)),
        }) #(.and_then(#stmt))*?;
    });
}

fn parse_attrs(attrs: &[Attribute]) -> (TokenStream, ModelField) {
    let mut bag = Vec::new();
    let mut model_field = ModelField::default();
    for attr in attrs.iter() {
        match attr.path.get_ident() {
            Some(ident) if ident == "field" => {
                if !attr.tokens.is_empty() {
                    model_field = attr.parse_args().unwrap();
                }
            }
            _ => bag.push(attr),
        }
    }
    (quote! {#(#bag)*}, model_field)
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
                let (attrs, model_field) = parse_attrs(&field.attrs);
                let variable: &Ident = field.ident.as_ref().unwrap();
                let variable_vis = &field.vis;
                let variable_key = if let Some(alias) = &model_field.alias {
                    let val = alias.value.value();
                    quote! { #val }
                } else {
                    let val = variable.to_string();
                    quote! { #val }
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
                if model_field.default.is_none() && !matches!(model_type, ModelType::Optional(_)) {
                    required.push(quote! { #variable_key })
                }
                match &model_type {
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
                        inner_type,
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
                    ModelType::Other => handle_other_type(
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

            let name = ident.to_string();
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

pub(crate) fn handle_enum(ident: Ident, vis: Visibility, data: DataEnum) -> TokenStream {
    let mut fields = Vec::new();
    let mut to_jsons = Vec::new();
    let mut statements = Vec::new();
    let mut schemas = Vec::new();
    for variant in data.variants {
        let variant_ident = variant.ident;
        let (attrs, model_field) = parse_attrs(&variant.attrs);

        match variant.fields {
            Fields::Named(field) => {
                if model_field.expected.is_some() {
                    panic!("Support only the expected term on the unit field.")
                }

                let mut fds = Vec::new();
                let mut maps = Vec::new();
                let mut idents = Vec::new();
                let mut properties = Vec::new();
                let mut required = Vec::new();
                let mut fd_statements = Vec::new();

                for fd in field.named {
                    let (fd_attrs, fd_model_field) = parse_attrs(&fd.attrs);
                    let fd_variable = fd.ident.unwrap();
                    let fd_variable_key = if let Some(alias) = &fd_model_field.alias {
                        let val = alias.value.value();
                        quote! { #val }
                    } else {
                        let val = fd_variable.to_string();
                        quote! { #val }
                    };
                    let fd_ty = &fd.ty;
                    let fd_model_type = ModelType::new(fd_ty);

                    let mut fd_conds: Vec<TokenStream> = Vec::from([quote! {
                        "title".to_string(),
                        dade::JsonValue::String(dade::ToTitle::to_title(#fd_variable_key))
                    }]);

                    match &fd_model_type {
                        ModelType::Null => handle_null_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Number => handle_number_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::String => handle_string_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Bool => handle_bool_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Optional(inner_type) => handle_optional_type(
                            inner_type,
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Array => handle_array_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Other => handle_other_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                    }

                    fds.push(quote! { #fd_attrs #fd_variable:#fd_ty });
                    idents.push(quote! { #fd_variable });
                    maps.push(quote! {
                        (#fd_variable_key.to_string(), dade::ToJsonValue::to_json_value(#fd_variable))
                    });
                    properties.push(quote! {
                        (
                            #fd_variable_key.to_string(),
                            {
                                let mut s = <#fd_ty as dade::RegisterSchema>::register_schema(defs);
                                if let dade::JsonValue::Object(ref mut dict) = s {
                                    #(dict.insert(#fd_conds));*;
                                }
                                s
                            }
                        )
                    });
                    if fd_model_field.default.is_none()
                        && !matches!(fd_model_type, ModelType::Optional(_))
                    {
                        required.push(quote! {
                            dade::JsonValue::String(#fd_variable_key.to_string())
                        })
                    }
                }
                fields.push(quote! { #attrs #variant_ident { #(#fds),* } });
                to_jsons.push(quote! {
                    #ident::#variant_ident{ #(#idents),* } => dade::JsonValue::Object(std::collections::BTreeMap::from([#(#maps),*]))
                });
                statements.push(quote! {
                    if let dade::JsonValue::Object(dict) = value {
                        let ret = || -> dade::Result<#ident> {
                            #(#fd_statements)*
                            Ok(#ident::#variant_ident { #(#idents),* })
                        };
                        if ret.is_ok() {
                            return ret
                        }
                    }
                });
                let title = format!("{}::{}", ident, variant_ident);
                schemas.push(quote! {
                    dade::JsonValue::Object(std::collections::BTreeMap::from([
                        (
                            "title".to_string(),
                            dade::JsonValue::String(#title.to_string()),
                        ),
                        (
                            "type".to_string(),
                            dade::JsonValue::String("object".to_string()),
                        ),
                        (
                            "properties".to_string(),
                            dade::JsonValue::Object(std::collections::BTreeMap::from([ #(#properties),* ])),
                        ),
                        (
                            "required".to_string(),
                            dade::JsonValue::Array(Vec::from([ #(#required),* ])),
                        ),
                    ]))
                });
            }
            Fields::Unnamed(field) => {
                if model_field.expected.is_some() {
                    panic!("Support only the expected term on the unnamed field.")
                }
                let mut fds = Vec::new();
                let mut keys = Vec::new();
                let mut properties = Vec::new();
                let mut fd_statements = Vec::new();
                for (idx, fd) in field.unnamed.iter().enumerate() {
                    let (fd_attrs, fd_model_field) = parse_attrs(&fd.attrs);
                    if fd_model_field.alias.is_some() {
                        panic!("No support alias on the unnamed field.")
                    };
                    let fd_variable = format_ident!("val{}", idx);
                    let fd_variable_key = quote!({ #idx });
                    let fd_ty = &fd.ty;
                    let fd_model_type = ModelType::new(fd_ty);
                    let mut fd_conds: Vec<TokenStream> = Vec::new();

                    match &fd_model_type {
                        ModelType::Null => handle_null_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Number => handle_number_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::String => handle_string_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Bool => handle_bool_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Optional(inner_type) => handle_optional_type(
                            inner_type,
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Array => handle_array_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                        ModelType::Other => handle_other_type(
                            &fd_model_field,
                            &fd_variable,
                            fd_ty,
                            &fd_variable_key,
                            &mut fd_statements,
                            &mut fd_conds,
                        ),
                    }

                    fds.push(quote! { #fd_attrs #fd_ty });
                    keys.push(fd_variable);
                    properties.push(quote! {
                        {
                            let mut s = <#fd_ty as dade::RegisterSchema>::register_schema(defs);
                            if let dade::JsonValue::Object(ref mut dict) = s {
                                #(dict.insert(#fd_conds));*;
                            }
                            s
                        }
                    });
                }
                fields.push(quote! { #attrs #variant_ident( #(#fds),* ) });
                if field.unnamed.len() == 1 {
                    to_jsons.push(quote! {
                        #ident::#variant_ident(#(#keys)*) => dade::ToJsonValue::to_json_value(#(#keys)*)
                    });
                    statements.push(quote! {
                        match dade::FromJsonValue::from_json_value(value) {
                            Ok(val) => {
                                let dict = [val];
                                let ret = || -> dade::Result<#ident> {
                                    #(#fd_statements)*
                                    Ok(#ident::#variant_ident { #(#keys),* })
                                };
                                if ret.is_ok() {
                                    return ret
                                }
                            }
                            Err(_) => {}
                        }
                    });
                    schemas.push(quote! { #(#properties)* });
                } else {
                    to_jsons.push(quote! {
                        #ident::#variant_ident(#(#keys),*) => {
                            dade::JsonValue::Array(Vec::from([#(dade::ToJsonValue::to_json_value(#keys)),*]))
                        }
                    });
                    statements.push(quote! {
                        if let dade::JsonValue::Array(dict) = value {
                            let ret = || -> dade::Result<#ident> {
                                #(#fd_statements)*
                                Ok(#ident::#variant_ident { #(#keys),* })
                            };
                            if ret.is_ok() {
                                return ret
                            }
                        }
                    });
                    let title = format!("{}::{}", ident, variant_ident);
                    schemas.push(quote! {
                        dade::JsonValue::Object(std::collections::BTreeMap::from([
                            ("title".to_string(), dade::JsonValue::String(#title.to_string())),
                            ("type".to_string(), dade::JsonValue::String("array".to_string())),
                            // TODO;
                            // ("items".to_string(), dade::JsonValue::Bool(false)),
                            ("prefixItems".to_string(), dade::JsonValue::Array(Vec::from([
                                #(#properties),*
                            ]))),
                        ]))
                    });
                }
            }
            Fields::Unit => {
                fields.push(quote! { #attrs #variant_ident });
                let cond = if let Some(expected) = model_field.expected {
                    expected.value.value().to_string()
                } else {
                    format!("{}", variant_ident)
                };
                to_jsons.push(quote! {
                        #ident::#variant_ident => dade::JsonValue::String(#cond.to_string())
                });
                statements.push(quote! {
                    if let dade::JsonValue::String(val) = value {
                        if val == #cond { return Ok(#ident::#variant_ident); }
                    }
                });
                schemas.push(quote! {
                    dade::JsonValue::Object(std::collections::BTreeMap::from([(
                        "const".to_string(),
                        dade::JsonValue::String(#cond.to_string()),
                    )]))
                });
            }
        };
    }
    let data_type = data.enum_token;
    let name = ident.to_string();
    let def_name = format!("#/definitions/{}", ident);
    quote! {
        #vis #data_type #ident { #(#fields),* }
        impl dade::ToJsonValue for #ident {
            fn to_json_value(&self) -> dade::JsonValue {
                match self { #(#to_jsons),* }
            }
        }
        impl dade::FromJsonValue for #ident {
            fn from_json_value(value: &dade::JsonValue) -> dade::Result<Self> {
                #(#statements)*
                Err(dade::Error::new_parse_err("No value with expected"))
            }
        }
        impl dade::RegisterSchema for #ident {
            fn register_schema(defs: &mut std::collections::BTreeMap<String, dade::JsonValue>) -> dade::JsonValue {
                if !defs.contains_key(&#name.to_string()) {
                    // Insert temporarily value.
                    defs.insert(#name.to_string(), dade::JsonValue::Null);
                    let json_value = dade::JsonValue::Array(Vec::from([ #(#schemas),*]));
                    // Swap to proper value.
                    defs.insert(
                        #name.to_string(),
                        dade::JsonValue::Object(std::collections::BTreeMap::from([
                            (
                                "title".to_string(),
                                dade::JsonValue::String(#name.to_string()),
                            ),
                            ("anyOf".to_string(), json_value),
                        ])),
                    );
                }
                dade::JsonValue::Object(std::collections::BTreeMap::from([(
                    "$ref".to_string(),
                    dade::JsonValue::String(#def_name.to_string()),
                )]))
            }
        }
    }
}
