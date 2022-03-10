use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    f32
);
fn main() {}
