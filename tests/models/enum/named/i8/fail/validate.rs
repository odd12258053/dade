use dade::{model, Result};
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: i8
    },
}
fn main() {}
