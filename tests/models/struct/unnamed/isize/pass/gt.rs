use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    isize
);
fn main() {}
