use dade::{model, Result};
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    usize
);
fn main() {}
