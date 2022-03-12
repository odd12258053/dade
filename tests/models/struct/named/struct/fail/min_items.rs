use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel {
    #[field(min_items = 2.0)]
    value: InnerModel,
}
fn main() {}
