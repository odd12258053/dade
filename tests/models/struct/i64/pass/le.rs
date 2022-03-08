use dade::model;
#[model]
struct TestModel {
    #[field(le = 2)]
    value: i64,
}
fn main() {}
