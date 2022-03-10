use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    isize
);
fn main() {}
