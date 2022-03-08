use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(expected = "val")]
        value: i8
    },
}
fn main() {}
