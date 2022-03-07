use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: Vec<()>,
}
fn main() {}
