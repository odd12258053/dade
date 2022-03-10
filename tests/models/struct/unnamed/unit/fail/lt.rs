use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    ()
);
fn main() {}
