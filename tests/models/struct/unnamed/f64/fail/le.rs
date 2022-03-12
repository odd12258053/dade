use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    f64
);
fn main() {}
