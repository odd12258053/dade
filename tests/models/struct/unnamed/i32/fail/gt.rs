use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2.0)]
    i32
);
fn main() {}
