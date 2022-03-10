use dade::{model, Result};
fn validate_fn(value: f64) -> Result<f64> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    f64
);
fn main() {}
