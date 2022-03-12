use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: i32,
}
fn main() {}
