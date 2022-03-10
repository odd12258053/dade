use dade::model;
#[model]
struct TestModel (
   #[field(default = 1.0)]
    u64
);
fn main() {}
