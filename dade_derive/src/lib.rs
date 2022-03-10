//!
//! If you want to write a model definition, this crate is needed. You need usage and more information, check this document [`dade`].
//!
//! [`dade`]: https://docs.rs/dade/latest/dade/
//!
//! ```rust
//! use dade::Model;
//! use dade_derive::model;
//!
//! #[model]
//! struct User {
//!     #[field(ge = 1)]
//!     id: u64,
//!     #[field(min_length = 1, max_length = 100)]
//!     name: String,
//!     #[field(default = "en")]
//!     lang: String,
//!     #[field(min_length = 1, max_length = 255, default = null)]
//!     url: Option<String>,
//!     #[field(default = false)]
//!     verified: bool,
//! }
//! ```
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput};

mod fields;
mod terms;
mod types;

/// This macro is to define a model.
#[proc_macro_attribute]
pub fn model(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let tokens = match input.data {
        Data::Struct(data) => types::handle_struct(input.ident, input.vis, input.attrs, data),
        Data::Enum(data) => types::handle_enum(input.ident, input.vis, input.attrs, data),
        _ => Err(syn::Error::new(
            input.span(),
            "Only support struct or enum.",
        )),
    }
    .unwrap_or_else(|err| err.to_compile_error());
    proc_macro::TokenStream::from(tokens)
}
