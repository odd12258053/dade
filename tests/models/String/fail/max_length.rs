use dade::model;
#[model]
struct TestModel {
    #[field(max_length = 2.0)]
    value: String,
}
fn main() {}
