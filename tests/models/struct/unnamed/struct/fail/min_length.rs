use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel (
   #[field(min_length = 2)]
    InnerModel
);
fn main() {}
