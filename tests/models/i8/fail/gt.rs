use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2.0)]
    value: i8,
}
fn main() {}
