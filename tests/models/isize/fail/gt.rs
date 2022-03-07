use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2.0)]
    value: isize,
}
fn main() {}
