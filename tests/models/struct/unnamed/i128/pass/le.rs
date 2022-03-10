use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    i128
);
fn main() {}
