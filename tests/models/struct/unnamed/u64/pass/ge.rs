use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    u64
);
fn main() {}
