use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(le = 2)]
        value: u128
    },
}
fn main() {}
