use dade::{model, Result};
fn validate_fn<T>(value: T) -> Result<T> {
    Ok(value)
}
#[model]
struct TestModel {
    #[field(validate = validate_fn)]
    value: i32,
}
fn main() {}
