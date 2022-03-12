use dade::model;
#[model]
enum TestModel {
    #[field(min_items = 2)]
    Value
}
fn main() {}
