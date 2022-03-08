use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        i64
    ),
}
fn main() {}
