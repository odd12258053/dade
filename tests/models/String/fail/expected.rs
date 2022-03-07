use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: String,
}
fn main() {}
