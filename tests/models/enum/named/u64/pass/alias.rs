use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: u64
    },
}
fn main() {}
