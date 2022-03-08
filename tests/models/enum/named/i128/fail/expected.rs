use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        i128
    ),
}
fn main() {}
