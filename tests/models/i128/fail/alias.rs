use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: i128,
}
fn main() {}
