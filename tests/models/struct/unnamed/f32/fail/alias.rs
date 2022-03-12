use dade::model;
#[model]
struct TestModel (
   #[field(alias = "val")]
    f32
);
fn main() {}
