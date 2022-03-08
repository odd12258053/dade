use dade::{model, Result};
fn validate_fn(value: u32) -> Result<u32> {
    Ok(value)
}
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        u32
    ),
}
fn main() {}
