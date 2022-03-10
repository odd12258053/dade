use dade::model;
#[model]
struct TestModel {
    #[field(le = 2)]
    value: u32,
}
fn main() {}
