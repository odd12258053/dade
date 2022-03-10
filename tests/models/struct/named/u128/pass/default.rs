use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: u128,
}
fn main() {}
