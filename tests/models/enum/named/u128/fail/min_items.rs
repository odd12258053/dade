use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(min_items = 2)]
        value: u128
    },
}
fn main() {}
