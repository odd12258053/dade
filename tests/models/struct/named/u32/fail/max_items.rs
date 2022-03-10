use dade::model;
#[model]
struct TestModel {
    #[field(max_items = 2)]
    value: u32,
}
fn main() {}
