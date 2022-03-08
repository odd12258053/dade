use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        i32
    ),
}
fn main() {}
