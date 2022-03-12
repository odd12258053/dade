use dade::model;
#[model]
struct TestModel (
   #[field(default = 1.0)]
    f64
);
fn main() {}
