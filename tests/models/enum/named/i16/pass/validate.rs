use dade::{model, Result};
fn validate_fn(value: i16) -> Result<i16> {
    Ok(value)
}
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: i16
    },
}
fn main() {}
