use dade::model;
#[model]
enum TestModel {
   #[field(ge = 2)]
    Value
}
fn main() {}
