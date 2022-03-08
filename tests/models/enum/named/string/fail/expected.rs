use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        String
    ),
}
fn main() {}
