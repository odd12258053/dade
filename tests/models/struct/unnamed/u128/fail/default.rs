use dade::model;
#[model]
struct TestModel (
   #[field(default = 1.0)]
    u128
);
fn main() {}
