use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    i128
);
fn main() {}
