use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        u16
    ),
}
fn main() {}
