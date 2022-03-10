use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    u32
);
fn main() {}
