use dade::model;
#[model]
struct TestModel (
   #[field(default = 1.0)]
    i32
);
fn main() {}
