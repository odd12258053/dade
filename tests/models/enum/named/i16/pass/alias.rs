use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: i16
    },
}
fn main() {}
