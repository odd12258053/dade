use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2.0)]
    usize
);
fn main() {}
