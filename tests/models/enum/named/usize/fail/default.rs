use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(default = 1.0)]
        value: usize
    },
}
fn main() {}
