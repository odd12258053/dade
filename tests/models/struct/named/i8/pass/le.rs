use dade::model;
#[model]
struct TestModel {
    #[field(le = 2)]
    value: i8,
}
fn main() {}
