use dade::model;
#[model]
struct TestModel {
    #[field(le = 2)]
    value: isize,
}
fn main() {}
