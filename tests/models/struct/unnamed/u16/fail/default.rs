use dade::model;
#[model]
struct TestModel (
   #[field(default = 1.0)]
    u16
);
fn main() {}
