use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_items = 2)]
        u32
    ),
}
fn main() {}
