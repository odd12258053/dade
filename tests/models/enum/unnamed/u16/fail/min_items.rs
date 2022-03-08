use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_items = 2)]
        u16
    ),
}
fn main() {}
