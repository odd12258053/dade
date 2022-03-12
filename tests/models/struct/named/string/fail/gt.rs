use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: String,
}
fn main() {}
