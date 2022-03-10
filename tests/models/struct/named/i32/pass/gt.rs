use dade::model;
#[model]
struct TestModel {
    #[field(gt = 2)]
    value: i32,
}
fn main() {}
