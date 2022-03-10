use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    i8
);
fn main() {}
