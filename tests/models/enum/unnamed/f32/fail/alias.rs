use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        f32
    ),
}
fn main() {}
