use dade::model;
#[model]
struct TestModel {
    #[field(default = 2.0)]
    value: i128,
}
fn main() {}
