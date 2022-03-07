use dade::model;
#[model]
struct TestModel {
    #[field(default = 2.0)]
    value: u128,
}
fn main() {}
