use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(gt = 2.0)]
        value: u64
    },
}
fn main() {}
