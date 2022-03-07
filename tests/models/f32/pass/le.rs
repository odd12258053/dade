use dade::model;
#[model]
struct TestModel {
    #[field(le = 2.0)]
    value: f32,
}
fn main() {}
