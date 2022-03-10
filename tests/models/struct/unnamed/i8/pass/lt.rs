use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    i8
);
fn main() {}
