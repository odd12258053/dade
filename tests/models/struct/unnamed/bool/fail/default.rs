use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    bool
);
fn main() {}
