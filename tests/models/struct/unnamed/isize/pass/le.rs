use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    isize
);
fn main() {}
