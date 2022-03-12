use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2.0)]
    f64
);
fn main() {}
