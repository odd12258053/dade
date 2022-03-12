use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = 1)]
        value: i8
    },
}
fn main() {}
