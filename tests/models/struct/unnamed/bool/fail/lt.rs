use dade::model;
#[model]
struct TestModel (
   #[field(lt = 2)]
    bool
);
fn main() {}
