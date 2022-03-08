use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2.0)]
    value: f64,
}
fn main() {}
