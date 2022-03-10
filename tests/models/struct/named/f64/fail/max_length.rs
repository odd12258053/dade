use dade::model;
#[model]
struct TestModel {
    #[field(max_length = 2)]
    value: f64,
}
fn main() {}
