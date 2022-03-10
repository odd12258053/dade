use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    f32
);
fn main() {}
