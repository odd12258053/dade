use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    Vec<()>
);
fn main() {}
