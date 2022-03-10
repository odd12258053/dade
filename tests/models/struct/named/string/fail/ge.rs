use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2)]
    value: String,
}
fn main() {}
