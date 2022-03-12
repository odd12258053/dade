use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    u32
);
fn main() {}
