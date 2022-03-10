use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2.0)]
    f32
);
fn main() {}
