use dade::{model, Result};
fn validate_fn(value: u128) -> Result<u128> {
    Ok(value)
}
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        u128
    ),
}
fn main() {}
