use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2.0)]
    f32
);
fn main() {}
