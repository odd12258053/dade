use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    i16
);
fn main() {}
