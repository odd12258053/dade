use dade::{model, Result};
fn validate_fn(value: i8) -> Result<i8> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    i8
);
fn main() {}
