use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        i16
    ),
}
fn main() {}
