use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        ()
    ),
}
fn main() {}
