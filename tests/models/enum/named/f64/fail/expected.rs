use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(expected = "val")]
        value: f64
    },
}
fn main() {}
