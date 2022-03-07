use dade::model;
#[model]
struct TestModel {
    #[field(expected = "value")]
    value: isize,
}
fn main() {}
