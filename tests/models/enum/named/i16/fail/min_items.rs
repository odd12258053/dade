use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_items = 2)]
        i16
    ),
}
fn main() {}
