use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    i16
);
fn main() {}
