use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    i32
);
fn main() {}
