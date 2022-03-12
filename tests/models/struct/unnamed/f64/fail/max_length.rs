use dade::model;
#[model]
struct TestModel (
   #[field(max_length = 2)]
    f64
);
fn main() {}
