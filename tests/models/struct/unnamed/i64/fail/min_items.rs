use dade::model;
#[model]
struct TestModel (
   #[field(min_items = 2)]
    i64
);
fn main() {}
