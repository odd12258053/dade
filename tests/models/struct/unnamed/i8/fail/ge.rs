use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2.0)]
    i8
);
fn main() {}
