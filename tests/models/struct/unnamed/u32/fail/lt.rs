use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2.0)]
    u32
);
fn main() {}
