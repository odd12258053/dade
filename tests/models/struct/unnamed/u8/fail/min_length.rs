use dade::model;
#[model]
struct TestModel (
   #[field(min_length = 2)]
    u8
);
fn main() {}
