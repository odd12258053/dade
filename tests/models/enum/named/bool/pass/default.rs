use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(default = false)]
        value: bool
    },
}
fn main() {}
