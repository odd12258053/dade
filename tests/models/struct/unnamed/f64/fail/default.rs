use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    f64
);
fn main() {}
