use dade::model;
#[model]
struct TestModel {
    #[field(default = null)]
    value: (),
}
fn main() {}
