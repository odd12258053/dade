use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(max_items = 2)]
        i8
    ),
}
fn main() {}
