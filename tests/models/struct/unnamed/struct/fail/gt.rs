use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel (
   #[field(gt = 2)]
    InnerModel
);
fn main() {}
