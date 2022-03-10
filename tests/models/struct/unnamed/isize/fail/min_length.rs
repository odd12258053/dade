use dade::model;
#[model]
struct TestModel (
   #[field(min_length = 2)]
    isize
);
fn main() {}
