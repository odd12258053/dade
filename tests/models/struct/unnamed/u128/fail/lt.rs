use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2.0)]
    u128
);
fn main() {}
