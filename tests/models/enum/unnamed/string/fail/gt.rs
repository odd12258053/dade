use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        String
    ),
}
fn main() {}
