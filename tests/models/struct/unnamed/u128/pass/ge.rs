use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    u128
);
fn main() {}
