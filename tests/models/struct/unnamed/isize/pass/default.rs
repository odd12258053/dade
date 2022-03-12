use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    isize
);
fn main() {}
