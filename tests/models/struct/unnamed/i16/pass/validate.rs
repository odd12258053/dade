use dade::{model, Result};
fn validate_fn(value: i16) -> Result<i16> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    i16
);
fn main() {}
