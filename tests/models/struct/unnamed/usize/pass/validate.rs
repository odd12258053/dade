use dade::{model, Result};
fn validate_fn(value: usize) -> Result<usize> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    usize
);
fn main() {}
