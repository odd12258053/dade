use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    i128
);
fn main() {}
