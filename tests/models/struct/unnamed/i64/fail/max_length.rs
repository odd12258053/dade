use dade::model;
#[model]
struct TestModel (
   #[field(max_length = 2)]
    i64
);
fn main() {}
