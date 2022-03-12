use dade::model;
#[model]
struct TestModel {
    #[field(validate = 2)]
    value: u8,
}
fn main() {}
