use dade::{model, Result};
fn validate_fn(value: i128) -> Result<i128> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    i128
);
fn main() {}
