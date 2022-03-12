use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel (
   #[field(max_length = 2)]
    InnerModel
);
fn main() {}
