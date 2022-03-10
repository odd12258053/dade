use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    bool
);
fn main() {}
