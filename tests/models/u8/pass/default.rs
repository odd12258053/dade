use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: u8,
}
fn main() {}
