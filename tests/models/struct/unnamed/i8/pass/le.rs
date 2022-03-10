use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    i8
);
fn main() {}
