use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    u128
);
fn main() {}
