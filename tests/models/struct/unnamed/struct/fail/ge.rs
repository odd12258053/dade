use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel (
    #[field(ge = 2)]
    InnerModel
);
fn main() {}
