use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(default = 1.0)]
        value: f64
    },
}
fn main() {}
