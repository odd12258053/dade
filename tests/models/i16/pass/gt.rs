use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: i16,
}
fn main() {}
