use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    u128
);
fn main() {}
