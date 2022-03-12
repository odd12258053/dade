use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        i8
    ),
}
fn main() {}
