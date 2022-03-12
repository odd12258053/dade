use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel {
    #[field(le = 2)]
    value: InnerModel,
}
fn main() {}
