use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: u64,
}
fn main() {}
