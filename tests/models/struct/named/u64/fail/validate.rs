use dade::model;
#[model]
struct TestModel {
    #[field(validate = 2)]
    value: u64,
}
fn main() {}
