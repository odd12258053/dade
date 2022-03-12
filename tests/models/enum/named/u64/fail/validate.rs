use dade::{model, Result};
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: u64
    },
}
fn main() {}
