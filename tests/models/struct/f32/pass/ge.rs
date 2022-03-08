use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2.0)]
    value: f32,
}
fn main() {}
