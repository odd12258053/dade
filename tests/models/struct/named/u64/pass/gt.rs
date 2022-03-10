use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: u64,
}
fn main() {}
