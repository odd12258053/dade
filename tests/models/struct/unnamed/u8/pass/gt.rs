use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    u8
);
fn main() {}
