use dade::model;
#[model]
struct TestModel {
    #[field(max_items = 2)]
    value: i128,
}
fn main() {}
