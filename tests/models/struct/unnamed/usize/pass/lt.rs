use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    usize
);
fn main() {}
