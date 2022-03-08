use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: i16,
}
fn main() {}
