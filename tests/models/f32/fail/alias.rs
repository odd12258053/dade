use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: f32,
}
fn main() {}
