use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    u64
);
fn main() {}
