use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2)]
    value: f64,
}
fn main() {}
