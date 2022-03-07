use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: f32,
}
fn main() {}
