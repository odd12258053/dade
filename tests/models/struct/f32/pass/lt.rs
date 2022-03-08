use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2.0)]
    value: f32,
}
fn main() {}
