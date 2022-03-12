use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    i8
);
fn main() {}
