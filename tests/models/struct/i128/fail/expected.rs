use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: i128,
}
fn main() {}
