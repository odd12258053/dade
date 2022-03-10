use dade::model;
#[model]
struct TestModel (
   #[field(max_length = 2)]
    i16
);
fn main() {}
