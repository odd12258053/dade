use dade::model;
#[model]
struct TestModel (
   #[field(min_items = 2)]
    i128
);
fn main() {}
