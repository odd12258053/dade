use dade::{model, Result};
fn validate_fn(value: u8) -> Result<u8> {
    Ok(value)
}
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        u8
    ),
}
fn main() {}
