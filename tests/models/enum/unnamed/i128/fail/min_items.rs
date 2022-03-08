use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_items = 2)]
        i128
    ),
}
fn main() {}
