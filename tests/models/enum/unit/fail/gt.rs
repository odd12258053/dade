use dade::model;
#[model]
enum TestModel {
   #[field(gt = 2)]
    Value
}
fn main() {}
