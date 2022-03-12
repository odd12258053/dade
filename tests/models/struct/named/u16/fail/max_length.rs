use dade::model;
#[model]
struct TestModel {
    #[field(max_length = 2)]
    value: u16,
}
fn main() {}
