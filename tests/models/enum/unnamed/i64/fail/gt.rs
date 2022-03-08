use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2.0)]
        i64
    ),
}
fn main() {}
