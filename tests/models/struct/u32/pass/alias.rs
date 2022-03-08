use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: u32,
}
fn main() {}
