use dade::model;
#[model]
struct TestModel (
   #[field(min_items = 2)]
    usize
);
fn main() {}
