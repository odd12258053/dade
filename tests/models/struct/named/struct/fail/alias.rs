use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel {
    #[field(alias = 2)]
    value: InnerModel,
}
fn main() {}
