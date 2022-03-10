use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel (
   #[field(alias = "val")]
    InnerModel
);
fn main() {}
