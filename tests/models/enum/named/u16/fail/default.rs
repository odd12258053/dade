use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(default = 1.0)]
        value: u16
    },
}
fn main() {}
