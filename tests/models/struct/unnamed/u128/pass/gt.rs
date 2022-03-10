use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    u128
);
fn main() {}
