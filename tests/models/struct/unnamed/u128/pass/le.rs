use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    u128
);
fn main() {}
