use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2.0)]
    usize
);
fn main() {}
