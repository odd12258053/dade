use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2.0)]
    u16
);
fn main() {}
