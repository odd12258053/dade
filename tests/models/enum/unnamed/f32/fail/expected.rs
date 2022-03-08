use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        f32
    ),
}
fn main() {}
