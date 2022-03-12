use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(ge = 2.0)]
        value: i128
    },
}
fn main() {}
