use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: i64,
}
fn main() {}
