use dade::model;
#[model]
struct TestModel (
   #[field(min_length = 2)]
    u128
);
fn main() {}
