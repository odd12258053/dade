use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        u128
    ),
}
fn main() {}
