use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: f32,
}
fn main() {}
