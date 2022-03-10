use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: i8
    },
}
fn main() {}
