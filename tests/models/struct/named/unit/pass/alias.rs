use dade::model;
#[model]
struct TestModel {
    #[field(alias = "val")]
    value: (),
}
fn main() {}
