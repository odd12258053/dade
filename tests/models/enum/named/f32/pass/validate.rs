use dade::{model, Result};
fn validate_fn(value: f32) -> Result<f32> {
    Ok(value)
}
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: f32
    },
}
fn main() {}
