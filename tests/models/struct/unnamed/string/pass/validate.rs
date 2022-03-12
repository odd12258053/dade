use dade::{model, Result};
fn validate_fn(value: String) -> Result<String> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    String
);
fn main() {}
