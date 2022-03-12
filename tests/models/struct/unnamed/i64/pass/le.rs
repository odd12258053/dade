use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    i64
);
fn main() {}
