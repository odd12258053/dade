use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(default = 1.0)]
        value: i64
    },
}
fn main() {}
