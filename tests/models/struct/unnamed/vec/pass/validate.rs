use dade::{model, Result};
fn validate_fn(value: Vec<()>) -> Result<Vec<()>> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    Vec<()>
);
fn main() {}
