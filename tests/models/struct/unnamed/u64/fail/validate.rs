use dade::{model, Result};
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    u64
);
fn main() {}
