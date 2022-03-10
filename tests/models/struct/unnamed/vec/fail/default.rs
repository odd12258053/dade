use dade::model;
#[model]
struct TestModel (
   #[field(default = 1)]
    Vec<()>
);
fn main() {}
