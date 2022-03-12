use dade::model;
#[model]
struct TestModel (
   #[field(default = 1.0)]
    usize
);
fn main() {}
