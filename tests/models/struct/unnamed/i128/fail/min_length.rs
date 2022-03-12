use dade::model;
#[model]
struct TestModel (
   #[field(min_length = 2)]
    i128
);
fn main() {}
