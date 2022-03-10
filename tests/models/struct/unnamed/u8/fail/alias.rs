use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    u8
);
fn main() {}
