use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: u32,
}
fn main() {}
