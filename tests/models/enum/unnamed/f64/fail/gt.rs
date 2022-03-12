use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        f64
    ),
}
fn main() {}
