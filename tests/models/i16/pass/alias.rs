use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: i16,
}
fn main() {}
