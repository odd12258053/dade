use dade::{model, Result};
fn validate_fn(value: i64) -> Result<i64> {
    Ok(value)
}
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        i64
    ),
}
fn main() {}
