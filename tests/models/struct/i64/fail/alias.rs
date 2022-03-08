use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: i64,
}
fn main() {}
