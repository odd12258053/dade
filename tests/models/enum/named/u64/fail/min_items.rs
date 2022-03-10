use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(min_items = 2)]
        value: u64
    },
}
fn main() {}
