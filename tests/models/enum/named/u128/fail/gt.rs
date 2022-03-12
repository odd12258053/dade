use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(gt = 2.0)]
        value: u128
    },
}
fn main() {}
