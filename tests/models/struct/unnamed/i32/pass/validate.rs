use dade::{model, Result};
fn validate_fn(value: i32) -> Result<i32> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    i32
);
fn main() {}