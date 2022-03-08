use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        u32
    ),
}
fn main() {}
