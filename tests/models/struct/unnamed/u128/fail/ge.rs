use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2.0)]
    u128
);
fn main() {}
