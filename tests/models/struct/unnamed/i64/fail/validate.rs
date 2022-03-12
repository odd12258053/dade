use dade::{model, Result};
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    i64
);
fn main() {}
