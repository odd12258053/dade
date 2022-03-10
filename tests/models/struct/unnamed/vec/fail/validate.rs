use dade::{model, Result};
#[model]
struct TestModel (
   #[field(validate = validate_fn)]
    Vec<()>
);
fn main() {}
