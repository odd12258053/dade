use dade::{model, Result};
fn validate_fn(value: bool) -> Result<bool> {
    Ok(value)
}
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        bool
    ),
}
fn main() {}
