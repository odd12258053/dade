use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2.0)]
    u16
);
fn main() {}
