use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        i64
    ),
}
fn main() {}
