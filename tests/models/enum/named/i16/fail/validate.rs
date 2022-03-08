use dade::{model, Result};
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        i16
    ),
}
fn main() {}
