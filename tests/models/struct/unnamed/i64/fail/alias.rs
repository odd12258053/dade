use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    i64
);
fn main() {}
