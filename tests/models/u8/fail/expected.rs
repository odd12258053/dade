use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: u8,
}
fn main() {}
