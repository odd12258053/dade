use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    u16
);
fn main() {}
