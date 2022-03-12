use dade::model;
#[model]
struct TestModel (
   #[field(le = 2.0)]
    u128
);
fn main() {}
