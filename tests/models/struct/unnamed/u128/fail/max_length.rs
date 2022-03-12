use dade::model;
#[model]
struct TestModel (
   #[field(max_length = 2)]
    u128
);
fn main() {}
