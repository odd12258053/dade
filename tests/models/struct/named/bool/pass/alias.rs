use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: bool,
}
fn main() {}
