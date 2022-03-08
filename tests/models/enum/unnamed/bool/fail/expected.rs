use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        bool
    ),
}
fn main() {}
