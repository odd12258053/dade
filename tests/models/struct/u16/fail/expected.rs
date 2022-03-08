use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: u16,
}
fn main() {}
