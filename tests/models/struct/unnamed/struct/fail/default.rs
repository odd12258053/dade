use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel (
   #[field(default = 1)]
    InnerModel
);
fn main() {}
