use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: u8
    },
}
fn main() {}
