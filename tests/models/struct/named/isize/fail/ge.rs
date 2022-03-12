use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2.0)]
    value: isize,
}
fn main() {}
