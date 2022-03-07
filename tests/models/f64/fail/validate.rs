use dade::model;
#[model]
struct TestModel {
    #[field(validate = 2)]
    value: f64,
}
fn main() {}
