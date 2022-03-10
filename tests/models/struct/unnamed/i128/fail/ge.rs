use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2.0)]
    i128
);
fn main() {}
