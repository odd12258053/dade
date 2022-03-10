use dade::model;
#[model]
struct TestModel (
   #[field(le = 2)]
    u8
);
fn main() {}
