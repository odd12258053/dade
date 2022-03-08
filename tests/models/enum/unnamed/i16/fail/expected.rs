use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        i16
    ),
}
fn main() {}
