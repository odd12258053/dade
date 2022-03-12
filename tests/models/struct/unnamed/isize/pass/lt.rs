use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    isize
);
fn main() {}
