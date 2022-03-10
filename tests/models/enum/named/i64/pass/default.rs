use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(default = 1)]
        value: i64
    },
}
fn main() {}
