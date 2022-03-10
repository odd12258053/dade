use dade::{model, Result};
fn validate_fn(value: Vec<()>) -> Result<Vec<()>> {
    Ok(value)
}
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: Vec<()>
    },
}
fn main() {}
