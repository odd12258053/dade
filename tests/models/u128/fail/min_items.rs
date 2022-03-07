use dade::model;
#[model]
struct TestModel {
    #[field(min_items = 2)]
    value: u128,
}
fn main() {}
