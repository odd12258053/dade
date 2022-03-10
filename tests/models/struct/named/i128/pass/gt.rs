use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: i128,
}
fn main() {}
