use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        usize
    ),
}
fn main() {}
