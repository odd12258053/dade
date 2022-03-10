use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    i32
);
fn main() {}
