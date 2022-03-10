use dade::model;
#[model]
struct TestModel {
    #[field(validate = 2)]
    value: i128,
}
fn main() {}
