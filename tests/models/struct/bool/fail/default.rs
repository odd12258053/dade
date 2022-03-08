use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: bool,
}
fn main() {}
