use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(default = "default")]
        value: String
    },
}
fn main() {}
