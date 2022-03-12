use dade::{model, Result};
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    bool
);
fn main() {}
