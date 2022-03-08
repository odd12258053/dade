use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = "val")]
        value: f64
    },
}
fn main() {}
