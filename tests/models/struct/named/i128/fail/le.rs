use dade::model;
#[model]
struct TestModel {
    #[field(le = 2.0)]
    value: i128,
}
fn main() {}
