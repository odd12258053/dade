use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(min_length = 2.0)]
        value: String
    },
}
fn main() {}
