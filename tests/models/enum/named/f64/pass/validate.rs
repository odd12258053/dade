use dade::{model, Result};
fn validate_fn(value: f64) -> Result<f64> {
    Ok(value)
}
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: f64
    },
}
fn main() {}
