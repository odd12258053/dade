use dade::model;
#[model]
struct TestModel (
   #[field(min_items = 2)]
    isize
);
fn main() {}
