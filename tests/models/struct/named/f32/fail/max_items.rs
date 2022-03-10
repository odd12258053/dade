use dade::model;
#[model]
struct TestModel {
    #[field(max_items = 2)]
    value: f32,
}
fn main() {}
