use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: Vec<()>,
}
fn main() {}
