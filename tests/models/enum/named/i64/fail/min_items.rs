use dade::model;
#[model]
enum TestModel {
    Value {
        #[field(min_items = 2)]
        value: i64
    },
}
fn main() {}
