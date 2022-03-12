use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: usize,
}
fn main() {}
