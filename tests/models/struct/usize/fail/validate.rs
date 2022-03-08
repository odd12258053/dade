use dade::model;
#[model]
struct TestModel {
    #[field(validate = 2)]
    value: usize,
}
fn main() {}
