use dade::model;
#[model]
struct TestModel {
    #[field(le = 2)]
    value: f64,
}
fn main() {}
