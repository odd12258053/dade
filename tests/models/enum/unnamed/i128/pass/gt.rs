use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        i128
    ),
}
fn main() {}
