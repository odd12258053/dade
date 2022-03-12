use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    f64
);
fn main() {}
