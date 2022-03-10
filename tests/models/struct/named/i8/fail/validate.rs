use dade::model;
#[model]
struct TestModel {
    #[field(validate = 2)]
    value: i8,
}
fn main() {}
