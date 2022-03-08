use dade::model;
#[model]
struct TestModel {
    #[field(le = 2.0)]
    value: u32,
}
fn main() {}
