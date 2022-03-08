use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        i8
    ),
}
fn main() {}
