use dade::{model, Result};
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    f32
);
fn main() {}
