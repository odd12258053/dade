use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    i128
);
fn main() {}
