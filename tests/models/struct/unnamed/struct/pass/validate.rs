use dade::{model, Result};
#[model]
struct InnerModel;
fn validate_fn(value: InnerModel) -> Result<InnerModel> {
    Ok(value)
}
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    InnerModel
);
fn main() {}
