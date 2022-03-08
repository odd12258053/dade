use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(expected = "val")]
        value: bool
    },
}
fn main() {}
