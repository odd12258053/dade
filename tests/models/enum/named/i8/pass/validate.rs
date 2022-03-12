use dade::{model, Result};
fn validate_fn(value: i8) -> Result<i8> {
    Ok(value)
}
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: i8
    },
}
fn main() {}
