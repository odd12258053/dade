use dade::model;
#[model]
struct TestModel {
    #[field(default = true)]
    value: bool,
}
fn main() {}
