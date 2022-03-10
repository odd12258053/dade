use dade::{model, Result};
fn validate_fn(value: u64) -> Result<u64> {
    Ok(value)
}
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: u64
    },
}
fn main() {}
