extern crate quote;

extern crate proc_macro;

mod terms;

use crate::terms::{AliasTerm, Condition, DefaultTerm, Term, ToToken, ValidateTerm};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Token, Type};

struct ModelField {
    default: Option<DefaultTerm>,
    validate: Option<ValidateTerm>,
    alias: Option<AliasTerm>,
    conditions: Vec<Condition>,
}

impl ModelField {
    fn default() -> Self {
        ModelField {
            default: None,
            validate: None,
            alias: None,
            conditions: Vec::new(),
        }
    }

    fn bool_condition(&self, _variable: &Ident) -> Result<TokenStream, &str> {
        if self.conditions.get(0).is_some() {
            return Err("Support condition is alias, default and validate");
        }
        Ok(quote! {})
    }
    fn number_condition(&self, variable: &Ident) -> Result<TokenStream, &str> {
        let mut terms = Vec::new();
        for cond in self.conditions.iter() {
            match cond {
                Condition::Gt(term) => terms.push(term.to_token(variable)),
                Condition::Ge(term) => terms.push(term.to_token(variable)),
                Condition::Lt(term) => terms.push(term.to_token(variable)),
                Condition::Le(term) => terms.push(term.to_token(variable)),
                _ => {
                    return Err("Support condition is gt, ge, lt, le, alias, default and validate")
                }
            }
        }
        Ok(if !terms.is_empty() {
            quote! {
                if !( #(#terms)&&* ) {
                    return Err(dade::Error::new("invalid number"))
                }
            }
        } else {
            quote! {}
        })
    }

    fn string_condition(&self, variable: &Ident) -> Result<TokenStream, &str> {
        let mut terms = Vec::new();
        for cond in self.conditions.iter() {
            match cond {
                Condition::MinLength(term) => terms.push(term.to_token(variable)),
                Condition::MaxLength(term) => terms.push(term.to_token(variable)),
                _ => {
                    return Err(
                        "Support condition is min_length, max_length, alias, default and validate",
                    )
                }
            }
        }
        Ok(if !terms.is_empty() {
            quote! {
                if !( #(#terms)&&* ) {
                    return Err(dade::Error::new("invalid string"))
                }
            }
        } else {
            quote! {}
        })
    }

    fn array_condition(&self, variable: &Ident) -> Result<TokenStream, &str> {
        let mut terms = Vec::new();
        for cond in self.conditions.iter() {
            match cond {
                Condition::MinLength(term) => terms.push(term.to_token(variable)),
                Condition::MaxLength(term) => terms.push(term.to_token(variable)),
                _ => {
                    return Err(
                        "Support condition is min_length, max_length, alias, default and validate",
                    )
                }
            }
        }
        Ok(if !terms.is_empty() {
            quote! {
                if !( #(#terms)&&* ) {
                    return Err(dade::Error::new("invalid array"))
                }
            }
        } else {
            quote! {}
        })
    }
}

impl Parse for ModelField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut default_term = None;
        let mut validate_term = None;
        let mut alias_term = None;
        let mut conditions = Vec::new();

        let terms: Punctuated<Term, Token![,]> = Punctuated::parse_terminated(input)?;
        for term in terms {
            match term {
                Term::MinLength(val) => {
                    conditions.push(Condition::MinLength(val));
                }
                Term::MaxLength(val) => {
                    conditions.push(Condition::MaxLength(val));
                }
                Term::MinItems(val) => {
                    conditions.push(Condition::MinItems(val));
                }
                Term::MaxItems(val) => {
                    conditions.push(Condition::MaxItems(val));
                }
                Term::Gt(val) => {
                    conditions.push(Condition::Gt(val));
                }
                Term::Ge(val) => {
                    conditions.push(Condition::Ge(val));
                }
                Term::Lt(val) => {
                    conditions.push(Condition::Lt(val));
                }
                Term::Le(val) => {
                    conditions.push(Condition::Le(val));
                }
                Term::Alias(val) => alias_term = Some(val),
                Term::Default(val) => default_term = Some(val),
                Term::Validate(val) => validate_term = Some(val),
            }
        }
        Ok(Self {
            conditions,
            default: default_term,
            validate: validate_term,
            alias: alias_term,
        })
    }
}

enum ModelType {
    Null,
    Number,
    String,
    Bool,
    Optional,
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
                        ModelType::Optional
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

#[proc_macro_attribute]
pub fn model(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let tokens = match input.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields_named) => {
                    let mut fields = Vec::new();
                    let mut maps = Vec::new();
                    let mut keys = Vec::new();
                    let mut statements = Vec::new();

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
                        match ModelType::new(ty) {
                            ModelType::Null => {
                                statements.push(quote! {
                                    // TODO: set a correct error.
                                    let #variable: #ty = match dict.get(#variable_key) {
                                        Some(val) => dade::FromJsonValue::from_json_value(val)?,
                                        None => (),
                                    };
                                });
                            }
                            ModelType::Number => {
                                let default_val = if let Some(term) = &model_field.default {
                                    let val = &term.value;
                                    quote! { #val }
                                } else {
                                    let msg = format!("not found key, {}", variable_key);
                                    quote! {
                                        return Err(dade::Error::new(#msg))
                                    }
                                };
                                statements.push(quote! {
                                    // TODO: set a correct error.
                                    let #variable: #ty = match dict.get(#variable_key) {
                                        Some(val) => dade::FromJsonValue::from_json_value(val)?,
                                        None => #default_val,
                                    };
                                });

                                statements.push(model_field.number_condition(variable).unwrap());
                                if let Some(term) = &model_field.validate {
                                    let fn_name = &term.value;
                                    statements.push(quote! {
                                        let #variable: #ty = #fn_name(#variable)?;
                                    });
                                }
                            }
                            ModelType::String => {
                                let default_val = if let Some(term) = &model_field.default {
                                    let val = &term.value;
                                    quote! { #val.to_string() }
                                } else {
                                    let msg = format!("not found key, {}", variable_key);
                                    quote! {
                                        return Err(dade::Error::new(#msg))
                                    }
                                };
                                statements.push(quote! {
                                    let #variable: #ty = match dict.get(#variable_key) {
                                        Some(val) => dade::FromJsonValue::from_json_value(val)?,
                                        None => #default_val,
                                    };
                                });

                                statements.push(model_field.string_condition(variable).unwrap());
                                if let Some(term) = model_field.validate {
                                    let fn_name = term.value;
                                    statements.push(quote! {
                                        let #variable: #ty = #fn_name(#variable)?;
                                    });
                                }
                            }
                            ModelType::Bool => {
                                let default_val = if let Some(term) = &model_field.default {
                                    let val = &term.value;
                                    quote! { #val }
                                } else {
                                    let msg = format!("not found key, {}", variable_key);
                                    quote! {
                                        return Err(dade::Error::new(#msg))
                                    }
                                };
                                statements.push(quote! {
                                    let #variable: #ty = match dict.get(#variable_key) {
                                        Some(val) => dade::FromJsonValue::from_json_value(val)?,
                                        None => #default_val,
                                    };
                                });

                                statements.push(model_field.bool_condition(variable).unwrap());
                                if let Some(term) = model_field.validate {
                                    let fn_name = term.value;
                                    statements.push(quote! {
                                        let #variable: #ty = #fn_name(#variable)?;
                                    });
                                }
                            }
                            ModelType::Optional => {
                                statements.push(quote! {
                                    let #variable: #ty = match dict.get(#variable_key) {
                                        Some(val) => dade::FromJsonValue::from_json_value(val)?,
                                        None => None,
                                    };
                                });
                                // TODO; handle condition
                                // statements.push(model_field.array_condition(variable).unwrap());
                                if let Some(term) = model_field.validate {
                                    let fn_name = term.value;
                                    statements.push(quote! {
                                        let #variable: #ty = #fn_name(#variable)?;
                                    });
                                }
                            }
                            ModelType::Array => {
                                let default_val = if let Some(term) = &model_field.default {
                                    let val = &term.value;
                                    quote! { #val }
                                } else {
                                    let msg = format!("not found key, {}", variable_key);
                                    quote! {
                                        return Err(dade::Error::new(#msg))
                                    }
                                };
                                statements.push(quote! {
                                    let #variable: #ty = match dict.get(#variable_key) {
                                        Some(val) => dade::FromJsonValue::from_json_value(val)?,
                                        None => #default_val,
                                    };
                                });

                                statements.push(model_field.array_condition(variable).unwrap());
                                if let Some(term) = model_field.validate {
                                    let fn_name = term.value;
                                    statements.push(quote! {
                                        let #variable: #ty = #fn_name(#variable)?;
                                    });
                                }
                            }
                            ModelType::Object => {
                                let msg = format!("not found key, {}", variable_key);
                                statements.push(quote! {
                                    let #variable: #ty = match dict.get(#variable_key) {
                                        Some(val) => dade::FromJsonValue::from_json_value(val)?,
                                        None => return Err(dade::Error::new(#msg)),
                                    };
                                });
                                if let Some(term) = model_field.validate {
                                    let fn_name = term.value;
                                    statements.push(quote! {
                                        let #variable: #ty = #fn_name(#variable)?;
                                    });
                                }
                            }
                        }
                        let colon_token = field.colon_token;
                        fields.push(quote! {#attrs #variable_vis #variable #colon_token #ty});
                    }

                    let ident = input.ident;
                    let vis = input.vis;
                    let data_type = data.struct_token;
                    quote! {
                        #vis #data_type #ident { #(#fields),* }
                        impl dade::ToJsonValue for #ident {
                            fn to_json_value(&self) -> dade::JsonValue {
                                dade::JsonValue::Object(
                                    indexmap::map::IndexMap::from( [#(#maps),*] )
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
                                    _ => Err(dade::Error::new("expect `JsonValue::Object`")),
                                }
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };
    // println!("{}", tokens);
    proc_macro::TokenStream::from(tokens)
}
