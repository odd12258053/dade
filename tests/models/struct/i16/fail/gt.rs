use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2.0)]
    value: i16,
}
fn main() {}
