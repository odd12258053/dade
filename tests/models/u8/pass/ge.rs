use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2)]
    value: u8,
}
fn main() {}
