use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2.0)]
    usize
);
fn main() {}
