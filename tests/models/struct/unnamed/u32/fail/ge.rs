use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2.0)]
    u32
);
fn main() {}
