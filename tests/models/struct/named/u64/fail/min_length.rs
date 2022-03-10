use dade::model;
#[model]
struct TestModel {
    #[field(min_length = 2)]
    value: u64,
}
fn main() {}
