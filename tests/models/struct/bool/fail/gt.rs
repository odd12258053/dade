use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: bool,
}
fn main() {}
