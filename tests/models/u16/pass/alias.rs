use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: u16,
}
fn main() {}
