use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    usize
);
fn main() {}
