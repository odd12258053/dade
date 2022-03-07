use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: f64,
}
fn main() {}
