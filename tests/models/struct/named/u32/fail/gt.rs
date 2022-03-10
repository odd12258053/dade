use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2.0)]
    value: u32,
}
fn main() {}
