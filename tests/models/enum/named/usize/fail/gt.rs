use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2.0)]
        usize
    ),
}
fn main() {}
