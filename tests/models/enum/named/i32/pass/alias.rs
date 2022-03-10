use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: i32
    },
}
fn main() {}
