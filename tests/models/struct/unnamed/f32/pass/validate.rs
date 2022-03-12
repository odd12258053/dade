use dade::{model, Result};
fn validate_fn(value: f32) -> Result<f32> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    f32
);
fn main() {}
