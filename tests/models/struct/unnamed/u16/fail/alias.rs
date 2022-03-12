use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    u16
);
fn main() {}
