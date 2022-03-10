use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(min_length = 2)]
        value: i128
    },
}
fn main() {}
