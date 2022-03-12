use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: i64
    },
}
fn main() {}
