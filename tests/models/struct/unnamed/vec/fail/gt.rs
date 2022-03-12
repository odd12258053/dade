use dade::model;
#[model]
struct TestModel (
   #[field(gt = 2)]
    Vec<()>
);
fn main() {}
