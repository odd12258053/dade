use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(gt = 2.0)]
        u8
    ),
}
fn main() {}
