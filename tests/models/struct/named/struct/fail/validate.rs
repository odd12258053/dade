use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel {
    #[field(validate = 2)]
    value: InnerModel,
}
fn main() {}
