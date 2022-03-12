use dade::model;
#[model]
struct TestModel {
    #[field(le = 2)]
    value: i128,
}
fn main() {}
