use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: i128,
}
fn main() {}
