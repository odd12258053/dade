use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2.0)]
        i32
    ),
}
fn main() {}
