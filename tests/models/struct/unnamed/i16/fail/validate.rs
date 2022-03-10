use dade::{model, Result};
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    i16
);
fn main() {}
