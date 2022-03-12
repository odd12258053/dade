use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(alias = 1)]
        value: u128
    },
}
fn main() {}
