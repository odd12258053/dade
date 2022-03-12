use dade::{model, Result};
#[model]
struct InnerModel;
fn validate_fn<T>(value: T) -> Result<T> {
    Ok(value)
}
#[model]
struct TestModel {
    #[field(validate = validate_fn)]
    value: InnerModel,
}
fn main() {}
