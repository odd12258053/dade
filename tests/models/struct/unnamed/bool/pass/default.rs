use dade::model;
#[model]
struct TestModel (
   #[field(default = false)]
    bool
);
fn main() {}
