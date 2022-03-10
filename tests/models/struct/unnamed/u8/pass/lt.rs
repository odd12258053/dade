use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    u8
);
fn main() {}
