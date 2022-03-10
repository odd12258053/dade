use dade::model;
#[model]
struct TestModel (
   #[field(le = 2.0)]
    u32
);
fn main() {}
