use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel {
    #[field(default = 2)]
    value: InnerModel,
}
fn main() {}
