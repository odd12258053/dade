use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel (
   #[field(le = 2)]
    InnerModel
);
fn main() {}
