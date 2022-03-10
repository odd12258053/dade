use dade::model;
#[model]
struct TestModel (
   #[field(min_items = 2.0)]
    Vec<()>
);
fn main() {}
