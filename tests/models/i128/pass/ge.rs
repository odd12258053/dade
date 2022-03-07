use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2)]
    value: i128,
}
fn main() {}
