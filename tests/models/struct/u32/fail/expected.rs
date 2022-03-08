use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: u32,
}
fn main() {}
