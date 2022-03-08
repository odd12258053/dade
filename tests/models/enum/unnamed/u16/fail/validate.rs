use dade::{model, Result};
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        u16
    ),
}
fn main() {}
