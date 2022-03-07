use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2)]
    value: u64,
}
fn main() {}
