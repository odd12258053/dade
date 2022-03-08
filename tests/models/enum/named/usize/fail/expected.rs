use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        usize
    ),
}
fn main() {}
