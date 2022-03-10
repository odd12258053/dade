use dade::model;
#[model]
struct TestModel {
    #[field(ge = 2)]
    value: (),
}
fn main() {}
