use dade::model;
#[model]
struct TestModel {
    #[field(le = 2.0)]
    value: f64,
}
fn main() {}
