use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: u8,
}
fn main() {}
