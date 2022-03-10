use dade::{model, Result};
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    u32
);
fn main() {}
