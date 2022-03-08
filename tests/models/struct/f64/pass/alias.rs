use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: f64,
}
fn main() {}
