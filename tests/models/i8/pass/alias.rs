use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: i8,
}
fn main() {}
