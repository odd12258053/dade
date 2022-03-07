use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: u128,
}
fn main() {}
