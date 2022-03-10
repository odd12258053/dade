use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: u128,
}
fn main() {}
