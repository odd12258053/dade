use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: u128,
}
fn main() {}
