use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2.0)]
    i128
);
fn main() {}
