use dade::{model, Result};
fn validate_fn(value: usize) -> Result<usize> {
    Ok(value)
}
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        usize
    ),
}
fn main() {}
