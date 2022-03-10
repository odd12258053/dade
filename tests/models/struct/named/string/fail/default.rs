use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: String,
}
fn main() {}
