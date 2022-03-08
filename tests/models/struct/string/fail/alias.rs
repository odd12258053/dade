use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: String,
}
fn main() {}
