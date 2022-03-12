use dade::model;
#[model]
struct TestModel {
    #[field(le = 2.0)]
    value: i64,
}
fn main() {}
