use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: u16,
}
fn main() {}
