use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: u32
    },
}
fn main() {}
