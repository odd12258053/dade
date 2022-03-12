use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = 1)]
        value: u16
    },
}
fn main() {}
