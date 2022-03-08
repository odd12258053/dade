use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: u32,
}
fn main() {}
