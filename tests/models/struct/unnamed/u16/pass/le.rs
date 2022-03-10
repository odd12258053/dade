use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    u16
);
fn main() {}
