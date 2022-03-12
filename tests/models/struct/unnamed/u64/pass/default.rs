use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    u64
);
fn main() {}
