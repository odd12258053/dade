use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2.0)]
    i64
);
fn main() {}
