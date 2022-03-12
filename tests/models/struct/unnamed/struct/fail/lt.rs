use dade::model;
#[model]
struct InnerModel;
#[model]
struct TestModel (
   #[field(lt = 2)]
    InnerModel
);
fn main() {}
