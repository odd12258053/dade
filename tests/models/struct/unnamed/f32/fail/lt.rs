use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    f32
);
fn main() {}
