use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2)]
    value: i32,
}
fn main() {}
