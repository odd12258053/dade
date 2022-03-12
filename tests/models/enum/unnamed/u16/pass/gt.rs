use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2)]
        u16
    ),
}
fn main() {}
