use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    i32
);
fn main() {}
