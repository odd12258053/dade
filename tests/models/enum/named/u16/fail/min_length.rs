use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(min_length = 2)]
        value: u16
    },
}
fn main() {}
