use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: u64,
}
fn main() {}
