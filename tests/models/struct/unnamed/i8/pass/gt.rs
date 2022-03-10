use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    i8
);
fn main() {}
