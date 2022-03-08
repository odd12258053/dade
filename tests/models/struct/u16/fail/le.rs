use dade::model;
#[model]
struct TestModel {
    #[field(le = 2.0)]
    value: u16,
}
fn main() {}
