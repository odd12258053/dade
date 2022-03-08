use dade::model;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: i16,
}
fn main() {}
