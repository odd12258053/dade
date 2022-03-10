use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2.0)]
    u16
);
fn main() {}
