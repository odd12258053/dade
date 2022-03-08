use dade::{model, Result};
fn validate_fn(value: bool) -> Result<bool> {
    Ok(value)
}
#[model]
struct TestModel {
    #[field(validate = validate_fn)]
    value: bool,
}
fn main() {}
