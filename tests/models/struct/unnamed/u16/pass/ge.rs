use dade::model;
#[model]
struct TestModel (
   #[field(ge = 2)]
    u16
);
fn main() {}
