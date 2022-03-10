use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(default = 1)]
        value: u64
    },
}
fn main() {}
