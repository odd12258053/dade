use dade::model;
#[model]
struct TestModel {
    #[field(le = 2.0)]
    value: usize,
}
fn main() {}
