use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel {
    #[field(ge = 2)]
    value: InnerModel,
}
fn main() {}
