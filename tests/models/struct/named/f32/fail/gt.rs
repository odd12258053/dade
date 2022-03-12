use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: f32,
}
fn main() {}
