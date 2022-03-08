use dade::model;
#[model]
struct TestModel {
    #[field(le = 2)]
    value: i32,
}
fn main() {}
