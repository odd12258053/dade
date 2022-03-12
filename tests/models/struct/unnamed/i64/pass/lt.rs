use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    i64
);
fn main() {}
