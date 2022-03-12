use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2.0)]
    value: f64,
}
fn main() {}
