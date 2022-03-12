use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    u32
);
fn main() {}
