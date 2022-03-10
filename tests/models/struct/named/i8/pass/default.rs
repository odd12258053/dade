use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: i8,
}
fn main() {}
