use dade::model;
#[model]
struct TestModel {
    #[field(default = "val")]
    value: String,
}
fn main() {}
