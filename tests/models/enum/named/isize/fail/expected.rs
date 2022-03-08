use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        isize
    ),
}
fn main() {}
