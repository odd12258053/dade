use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: f64,
}
fn main() {}
