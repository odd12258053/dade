use dade::model;
#[model]
struct TestModel {
    #[field(max_items = 2)]
    value: f64,
}
fn main() {}
