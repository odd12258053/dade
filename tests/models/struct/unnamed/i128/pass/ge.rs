use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    i128
);
fn main() {}
