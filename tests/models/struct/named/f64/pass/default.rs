use dade::model;
#[model]
struct TestModel {
    #[field(default = 2.0)]
    value: f64,
}
fn main() {}
