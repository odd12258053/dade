use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: u128
    },
}
fn main() {}
