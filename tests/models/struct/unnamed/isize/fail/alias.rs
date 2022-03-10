use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    isize
);
fn main() {}
