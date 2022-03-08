use dade::{model, Result};
#[model]
enum TestModel {
    Value {
        #[field(validate = validate_fn)]
        value: f64
    },
}
fn main() {}
