use dade::model;
#[model]
struct TestModel (
   #[field(default = 1.0)]
    i128
);
fn main() {}
