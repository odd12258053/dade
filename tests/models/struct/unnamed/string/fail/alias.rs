use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    String
);
fn main() {}
