use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(ge = 2)]
        value: f64
    },
}
fn main() {}
