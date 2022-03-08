use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: u64,
}
fn main() {}
