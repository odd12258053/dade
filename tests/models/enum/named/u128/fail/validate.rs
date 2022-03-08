use dade::{model, Result};
#[model]
enum TestModel {
    Value(
       #[field(validate = validate_fn)]
        u128
    ),
}
fn main() {}
