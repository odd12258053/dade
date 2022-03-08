use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        u64
    ),
}
fn main() {}
