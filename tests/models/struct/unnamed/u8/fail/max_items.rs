use dade::model;
#[model]
struct TestModel (
   #[field(max_items = 2)]
    u8
);
fn main() {}
