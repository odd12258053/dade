use dade::model;
#[model]
struct TestModel (
   #[field(max_items = 2)]
    bool
);
fn main() {}
