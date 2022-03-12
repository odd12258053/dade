use dade::{model, Result};
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    i32
);
fn main() {}
