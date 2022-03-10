use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    f32
);
fn main() {}
