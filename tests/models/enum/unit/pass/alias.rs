use dade::model;
#[model]
enum TestModel {
    #[field(alias = "val")]
    Value
}
fn main() {}
