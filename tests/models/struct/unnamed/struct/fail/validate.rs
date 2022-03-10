use dade::{model, Result};
#[model]
struct InnerModel;
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    InnerModel
);
fn main() {}
