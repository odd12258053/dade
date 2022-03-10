use dade::model;
#[model]
struct TestModel (
   #[field(le = 2.0)]
    f32
);
fn main() {}
