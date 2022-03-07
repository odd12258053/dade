use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2.0)]
    value: u128,
}
fn main() {}
