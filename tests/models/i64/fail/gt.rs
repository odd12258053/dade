use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2.0)]
    value: i64,
}
fn main() {}
