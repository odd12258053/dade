use dade::model;
#[model]
enum TestModel {
    #[field(max_items = 2)]
    Value
}
fn main() {}
