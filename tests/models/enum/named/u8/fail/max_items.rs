use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(max_items = 2)]
        u8
    ),
}
fn main() {}
