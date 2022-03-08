use dade::model;
#[model]
struct TestModel {
    #[field(default = 2.0)]
    value: i64,
}
fn main() {}
