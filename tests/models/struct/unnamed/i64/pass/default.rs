use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    i64
);
fn main() {}
