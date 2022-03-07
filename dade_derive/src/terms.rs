use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Lit, LitFloat, LitInt, LitStr, Token};

pub(crate) trait ToValidateToken {
    fn to_validate_token(&self, variable: &Ident) -> TokenStream;
}

pub(crate) trait ToSchema {
    fn to_schema(&self) -> TokenStream;
}

pub(crate) struct MinLengthTerm {
    pub value: LitInt,
}

impl ToValidateToken for MinLengthTerm {
    fn to_validate_token(&self, variable: &Ident) -> TokenStream {
        let val = &self.value;
        let msg = format!(
            "the length of {} must be equal to or greater than {}",
            variable, val
        );
        quote! {
            | value | {
                if value.len() >= #val {
                    Ok(value)
                } else {
                    Err(dade::Error::validate_err(#msg))
                }
            }
        }
    }
}

impl ToSchema for MinLengthTerm {
    fn to_schema(&self) -> TokenStream {
        let val = &self.value;
        quote! {
            "minLength".to_string(),
            dade::JsonValue::Number(dade::Number::from(#val))
        }
    }
}

pub(crate) struct MaxLengthTerm {
    pub value: LitInt,
}

impl ToValidateToken for MaxLengthTerm {
    fn to_validate_token(&self, variable: &Ident) -> TokenStream {
        let val = &self.value;
        let msg = format!(
            "the length of {} must be equal to or less than {}",
            variable, val
        );
        quote! {
            | value | {
                if value.len() <= #val {
                    Ok(value)
                } else {
                    Err(dade::Error::validate_err(#msg))
                }
            }
        }
    }
}

impl ToSchema for MaxLengthTerm {
    fn to_schema(&self) -> TokenStream {
        let val = &self.value;
        quote! {
            "maxLength".to_string(),
            dade::JsonValue::Number(dade::Number::from(#val))
        }
    }
}

pub(crate) struct MinItemsTerm {
    pub value: LitInt,
}

impl ToValidateToken for MinItemsTerm {
    fn to_validate_token(&self, variable: &Ident) -> TokenStream {
        let val = &self.value;
        let msg = format!(
            "the number of items in {} must be equal to or greater than {}",
            variable, val
        );
        quote! {
            | value | {
                if value.len() >= #val {
                    Ok(value)
                } else {
                    Err(dade::Error::validate_err(#msg))
                }
            }
        }
    }
}

impl ToSchema for MinItemsTerm {
    fn to_schema(&self) -> TokenStream {
        let val = &self.value;
        quote! {
            "minItems".to_string(),
            dade::JsonValue::Number(dade::Number::from(#val))
        }
    }
}

pub(crate) struct MaxItemsTerm {
    pub value: LitInt,
}

impl ToValidateToken for MaxItemsTerm {
    fn to_validate_token(&self, variable: &Ident) -> TokenStream {
        let val = &self.value;
        let msg = format!(
            "the number of items in {} must be equal to or less than {}",
            variable, val
        );
        quote! {
            | value | {
                if value.len() <= #val {
                    Ok(value)
                } else {
                    Err(dade::Error::validate_err(#msg))
                }
            }
        }
    }
}
impl ToSchema for MaxItemsTerm {
    fn to_schema(&self) -> TokenStream {
        let val = &self.value;
        quote! {
            "maxItems".to_string(),
            dade::JsonValue::Number(dade::Number::from(#val))
        }
    }
}

pub(crate) enum LitNumber {
    Int(LitInt),
    Float(LitFloat),
}

impl Parse for LitNumber {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit: Lit = input.parse()?;
        match lit {
            Lit::Int(val) => Ok(LitNumber::Int(val)),
            Lit::Float(val) => Ok(LitNumber::Float(val)),
            _ => Err(input.error("Un support Type")),
        }
    }
}

impl quote::ToTokens for LitNumber {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LitNumber::Int(val) => val.to_tokens(tokens),
            LitNumber::Float(val) => val.to_tokens(tokens),
        }
    }
}

impl ToString for LitNumber {
    fn to_string(&self) -> String {
        match self {
            LitNumber::Int(val) => val.to_string(),
            LitNumber::Float(val) => val.to_string(),
        }
    }
}

pub(crate) struct GtTerm {
    pub value: LitNumber,
}

impl ToValidateToken for GtTerm {
    fn to_validate_token(&self, variable: &Ident) -> TokenStream {
        let val = &self.value;
        let msg = format!("{} must be greater than {}", variable, val.to_string());
        quote! {
            | value | {
                if value > #val {
                    Ok(value)
                } else {
                    Err(dade::Error::validate_err(#msg))
                }
            }
        }
    }
}

impl ToSchema for GtTerm {
    fn to_schema(&self) -> TokenStream {
        let val = &self.value;
        quote! {
            "exclusiveMinimum".to_string(),
            dade::JsonValue::Number(dade::Number::from(#val))
        }
    }
}

pub(crate) struct GeTerm {
    pub value: LitNumber,
}

impl ToValidateToken for GeTerm {
    fn to_validate_token(&self, variable: &Ident) -> TokenStream {
        let val = &self.value;
        let msg = format!(
            "{} must be equal to or greater than {}",
            variable,
            val.to_string()
        );
        quote! {
            | value | {
                if value >= #val {
                    Ok(value)
                } else {
                    Err(dade::Error::validate_err(#msg))
                }
            }
        }
    }
}

impl ToSchema for GeTerm {
    fn to_schema(&self) -> TokenStream {
        let val = &self.value;
        quote! {
            "minimum".to_string(),
            dade::JsonValue::Number(dade::Number::from(#val))
        }
    }
}

pub(crate) struct LtTerm {
    pub value: LitNumber,
}

impl ToValidateToken for LtTerm {
    fn to_validate_token(&self, variable: &Ident) -> TokenStream {
        let val = &self.value;
        let msg = format!("{} must be less than {}", variable, val.to_string());
        quote! {
            | value | {
                if value < #val {
                    Ok(value)
                } else {
                    Err(dade::Error::validate_err(#msg))
                }
            }
        }
    }
}

impl ToSchema for LtTerm {
    fn to_schema(&self) -> TokenStream {
        let val = &self.value;
        quote! {
            "exclusiveMaximum".to_string(),
            dade::JsonValue::Number(dade::Number::from(#val))
        }
    }
}

pub(crate) struct LeTerm {
    pub value: LitNumber,
}

impl ToValidateToken for LeTerm {
    fn to_validate_token(&self, variable: &Ident) -> TokenStream {
        let val = &self.value;
        let msg = format!(
            "{} must be equal to or less than {}",
            variable,
            val.to_string()
        );
        quote! {
            | value | {
                if value <= #val {
                    Ok(value)
                } else {
                    Err(dade::Error::validate_err(#msg))
                }
            }
        }
    }
}

impl ToSchema for LeTerm {
    fn to_schema(&self) -> TokenStream {
        let val = &self.value;
        quote! {
            "maximum".to_string(),
            dade::JsonValue::Number(dade::Number::from(#val))
        }
    }
}

pub(crate) struct AliasTerm {
    pub value: LitStr,
}

pub(crate) struct IdentDefaultTerm {
    pub value: Ident,
}

pub(crate) struct LitDefaultTerm {
    pub value: Lit,
}

pub(crate) enum DefaultTerm {
    Ident(IdentDefaultTerm),
    Lit(LitDefaultTerm),
}

impl Parse for DefaultTerm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            return Ok(DefaultTerm::Ident(IdentDefaultTerm { value: ident }));
        }
        let lit: Lit = input.parse()?;
        Ok(DefaultTerm::Lit(LitDefaultTerm { value: lit }))
    }
}

pub(crate) struct ValidateTerm {
    pub value: Ident,
}

pub(crate) struct ExpectedTerm {
    pub value: LitStr,
}

pub(crate) enum Term {
    MinLength(MinLengthTerm),
    MaxLength(MaxLengthTerm),
    MinItems(MinItemsTerm),
    MaxItems(MaxItemsTerm),
    Gt(GtTerm),
    Ge(GeTerm),
    Lt(LtTerm),
    Le(LeTerm),
    Alias(AliasTerm),
    Default(DefaultTerm),
    Validate(ValidateTerm),
    Expected(ExpectedTerm),
}

impl Parse for Term {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input
            .parse()
            .map(|ident: Ident| ident.to_string())
            .or_else(|_| {
                input
                    .parse()
                    .map(|_: Token![default]| "default".to_string())
            })?;
        let _eq_token: Token![=] = input.parse()?;
        if ident == "min_length" {
            Ok(Term::MinLength(MinLengthTerm {
                value: input.parse()?,
            }))
        } else if ident == "max_length" {
            Ok(Term::MaxLength(MaxLengthTerm {
                value: input.parse()?,
            }))
        } else if ident == "min_items" {
            Ok(Term::MinItems(MinItemsTerm {
                value: input.parse()?,
            }))
        } else if ident == "max_items" {
            Ok(Term::MaxItems(MaxItemsTerm {
                value: input.parse()?,
            }))
        } else if ident == "gt" {
            Ok(Term::Gt(GtTerm {
                value: input.parse()?,
            }))
        } else if ident == "ge" {
            Ok(Term::Ge(GeTerm {
                value: input.parse()?,
            }))
        } else if ident == "lt" {
            Ok(Term::Lt(LtTerm {
                value: input.parse()?,
            }))
        } else if ident == "le" {
            Ok(Term::Le(LeTerm {
                value: input.parse()?,
            }))
        } else if ident == "alias" {
            Ok(Term::Alias(AliasTerm {
                value: input.parse()?,
            }))
        } else if ident == "default" {
            let term: DefaultTerm = input.parse()?;
            Ok(Term::Default(term))
        } else if ident == "validate" {
            Ok(Term::Validate(ValidateTerm {
                value: input.parse()?,
            }))
        } else if ident == "expected" {
            Ok(Term::Expected(ExpectedTerm {
                value: input.parse()?,
            }))
        } else {
            Err(input.error("un support type"))
        }
    }
}

pub(crate) enum Condition {
    MinLength(MinLengthTerm),
    MaxLength(MaxLengthTerm),
    MinItems(MinItemsTerm),
    MaxItems(MaxItemsTerm),
    Gt(GtTerm),
    Ge(GeTerm),
    Lt(LtTerm),
    Le(LeTerm),
}
