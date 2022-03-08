use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        u128
    ),
}
fn main() {}
