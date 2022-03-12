use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2.0)]
    i16
);
fn main() {}
