use dade::model;
#[model]
struct TestModel (
   #[field(default = 1.0)]
    f32
);
fn main() {}
