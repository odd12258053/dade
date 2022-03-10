use dade::model;
#[model]
struct TestModel (
   #[field(max_length = 2.0)]
    String
);
fn main() {}
