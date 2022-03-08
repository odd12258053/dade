use dade::model;
#[model]
struct TestModel {
    #[field(le = 2.0)]
    value: i32,
}
fn main() {}
