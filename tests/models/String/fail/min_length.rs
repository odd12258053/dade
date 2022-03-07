use dade::model;
#[model]
struct TestModel {
    #[field(min_length = 2.0)]
    value: String,
}
fn main() {}
