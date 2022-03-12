use dade::{model, Result};
fn validate_fn(value: u16) -> Result<u16> {
    Ok(value)
}
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        u16
    ),
}
fn main() {}
