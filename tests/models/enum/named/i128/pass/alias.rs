use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: i128
    },
}
fn main() {}
