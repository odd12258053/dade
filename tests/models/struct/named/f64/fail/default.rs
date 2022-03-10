use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: f64,
}
fn main() {}
