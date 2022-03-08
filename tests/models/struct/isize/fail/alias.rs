use dade::model;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: isize,
}
fn main() {}
