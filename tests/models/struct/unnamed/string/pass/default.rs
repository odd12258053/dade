use dade::model;
#[model]
struct TestModel (
   #[field(default = "default")]
    String
);
fn main() {}
