use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    Vec<()>
);
fn main() {}
