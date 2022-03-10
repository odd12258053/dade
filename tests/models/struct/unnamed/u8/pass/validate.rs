use dade::{model, Result};
fn validate_fn(value: u8) -> Result<u8> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    u8
);
fn main() {}
