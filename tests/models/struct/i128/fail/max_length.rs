use dade::model;
#[model]
struct TestModel {
    #[field(max_length = 2)]
    value: i128,
}
fn main() {}
