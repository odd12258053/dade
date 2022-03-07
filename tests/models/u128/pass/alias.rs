use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: u128,
}
fn main() {}
