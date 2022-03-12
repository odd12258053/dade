use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    i32
);
fn main() {}
