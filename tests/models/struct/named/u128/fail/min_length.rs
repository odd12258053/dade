use dade::model;
#[model]
struct TestModel {
    #[field(min_length = 2)]
    value: u128,
}
fn main() {}
