use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    String
);
fn main() {}
