use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    f64
);
fn main() {}
