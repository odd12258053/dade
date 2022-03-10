use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: u8,
}
fn main() {}
