use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    u64
);
fn main() {}
