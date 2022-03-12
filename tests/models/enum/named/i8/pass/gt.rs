use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(gt = 2)]
        value: i8
    },
}
fn main() {}
