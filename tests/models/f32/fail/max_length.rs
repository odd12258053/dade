use dade::model;
#[model]
struct TestModel {
    #[field(max_length = 2)]
    value: f32,
}
fn main() {}
