use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: u64,
}
fn main() {}
