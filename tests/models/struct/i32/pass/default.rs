use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: i32,
}
fn main() {}
