use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: String,
}
fn main() {}
