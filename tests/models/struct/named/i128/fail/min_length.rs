use dade::model;
#[model]
struct TestModel {
    #[field(min_length = 2)]
    value: i128,
}
fn main() {}
