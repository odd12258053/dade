use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: InnerModel,
}
fn main() {}
