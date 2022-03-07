use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: usize,
}
fn main() {}
