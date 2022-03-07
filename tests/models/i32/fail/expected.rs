use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: i32,
}
fn main() {}
