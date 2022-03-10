use dade::model;
#[model]
struct TestModel (
   #[field(max_items = 2)]
    u128
);
fn main() {}
