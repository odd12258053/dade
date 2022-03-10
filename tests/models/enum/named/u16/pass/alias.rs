use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: u16
    },
}
fn main() {}
