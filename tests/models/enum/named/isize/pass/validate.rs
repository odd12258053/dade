use dade::{model, Result};
fn validate_fn(value: isize) -> Result<isize> {
    Ok(value)
}
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: isize
    },
}
fn main() {}
