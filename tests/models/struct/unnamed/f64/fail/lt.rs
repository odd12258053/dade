use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    f64
);
fn main() {}
