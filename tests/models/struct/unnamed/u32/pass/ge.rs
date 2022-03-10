use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    u32
);
fn main() {}
