use dade::model;
#[model]
struct TestModel (
   #[field(max_length = 2)]
    u8
);
fn main() {}
