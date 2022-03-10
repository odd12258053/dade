use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    u32
);
fn main() {}
