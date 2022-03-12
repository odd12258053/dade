use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2.0)]
    u128
);
fn main() {}
