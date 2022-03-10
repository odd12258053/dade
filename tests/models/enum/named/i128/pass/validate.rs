use dade::{model, Result};
fn validate_fn(value: i128) -> Result<i128> {
    Ok(value)
}
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: i128
    },
}
fn main() {}
