use dade::model;
#[model]
struct TestModel {
    #[field(le = 2)]
    value: u128,
}
fn main() {}
