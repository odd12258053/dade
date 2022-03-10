use dade::model;
#[model]
struct TestModel (
   #[field(max_items = 2)]
    String
);
fn main() {}
