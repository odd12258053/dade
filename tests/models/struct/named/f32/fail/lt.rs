use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: f32,
}
fn main() {}
