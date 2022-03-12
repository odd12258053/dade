use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_items = 2)]
        u128
    ),
}
fn main() {}
