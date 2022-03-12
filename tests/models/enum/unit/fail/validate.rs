use dade::{model, Result};
#[model]
enum TestModel {
    #[field(validate = validate_fn)]
    Value
}
fn main() {}
