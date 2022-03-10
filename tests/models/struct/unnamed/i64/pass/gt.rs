use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    i64
);
fn main() {}
