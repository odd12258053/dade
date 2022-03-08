use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: i8,
}
fn main() {}
