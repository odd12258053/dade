use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2.0)]
    u64
);
fn main() {}