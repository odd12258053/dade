use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    f32
);
fn main() {}
