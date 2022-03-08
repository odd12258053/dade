use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        u64
    ),
}
fn main() {}
