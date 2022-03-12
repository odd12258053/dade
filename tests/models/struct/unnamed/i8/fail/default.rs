use dade::model;
#[model]
struct TestModel (
   #[field(default = 1.0)]
    i8
);
fn main() {}
