use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2.0)]
    value: i64,
}
fn main() {}
