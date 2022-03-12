use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: f32,
}
fn main() {}
