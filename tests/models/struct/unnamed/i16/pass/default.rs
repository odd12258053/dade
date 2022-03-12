use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    i16
);
fn main() {}
