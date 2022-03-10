use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    u32
);
fn main() {}
