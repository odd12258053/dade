use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(ge = 2)]
        value: String
    },
}
fn main() {}
