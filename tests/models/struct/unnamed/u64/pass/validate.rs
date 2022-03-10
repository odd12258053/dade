use dade::{model, Result};
fn validate_fn(value: u64) -> Result<u64> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    u64
);
fn main() {}
