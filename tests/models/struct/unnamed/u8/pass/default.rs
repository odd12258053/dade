use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    u8
);
fn main() {}
