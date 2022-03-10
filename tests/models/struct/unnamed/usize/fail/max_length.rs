use dade::model;
#[model]
struct TestModel (
   #[field(max_length = 2)]
    usize
);
fn main() {}
