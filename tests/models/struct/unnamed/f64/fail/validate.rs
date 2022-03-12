use dade::{model, Result};
#[model]
struct TestModel (
    #[field(validate = validate_fn)]
    f64
);
fn main() {}
