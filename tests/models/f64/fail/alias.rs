use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: f64,
}
fn main() {}
