use dade::model;
#[model]
struct TestModel (
   #[field(min_items = 2)]
    i32
);
fn main() {}
