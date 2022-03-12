use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    usize
);
fn main() {}
