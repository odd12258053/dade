use dade::{model, Result};
fn validate_fn(value: isize) -> Result<isize> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    isize
);
fn main() {}
