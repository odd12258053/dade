use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    i128
);
fn main() {}
