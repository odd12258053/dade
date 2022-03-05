use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;

use crate::terms::{AliasTerm, Condition, DefaultTerm, ExpectedTerm, Term, ValidateTerm};

pub(crate) struct ModelField {
    pub(crate) default: Option<DefaultTerm>,
    pub(crate) validate: Option<ValidateTerm>,
    pub(crate) alias: Option<AliasTerm>,
    pub(crate) expected: Option<ExpectedTerm>,
    pub(crate) conditions: Vec<Condition>,
}

impl ModelField {
    pub(crate) fn default() -> Self {
        ModelField {
            default: None,
            validate: None,
            alias: None,
            expected: None,
            conditions: Vec::new(),
        }
    }
}

impl Parse for ModelField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut default_term = None;
        let mut validate_term = None;
        let mut alias_term = None;
        let mut expected_term = None;
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
                Term::Expected(val) => expected_term = Some(val),
            }
        }
        Ok(Self {
            conditions,
            default: default_term,
            validate: validate_term,
            alias: alias_term,
            expected: expected_term,
        })
    }
}
