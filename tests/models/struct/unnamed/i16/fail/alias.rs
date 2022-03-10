use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    i16
);
fn main() {}
