use dade::model;
#[model]
struct TestModel (
   #[field(max_length = 2)]
    u16
);
fn main() {}
