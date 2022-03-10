use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(gt = 2)]
        value: i128
    },
}
fn main() {}
