use dade::model;
#[model]
struct TestModel {
    #[field(min_items = 2)]
    value: u64,
}
fn main() {}
