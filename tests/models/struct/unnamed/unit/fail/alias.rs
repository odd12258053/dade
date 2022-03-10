use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
   ()
);
fn main() {}
