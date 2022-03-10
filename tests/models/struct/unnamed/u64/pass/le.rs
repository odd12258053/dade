use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    u64
);
fn main() {}
