use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    i8
);
fn main() {}
