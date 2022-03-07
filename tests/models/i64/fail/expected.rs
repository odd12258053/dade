use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: i64,
}
fn main() {}
