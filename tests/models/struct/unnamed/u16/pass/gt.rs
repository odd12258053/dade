use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    u16
);
fn main() {}
