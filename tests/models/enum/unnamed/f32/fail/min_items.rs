use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_items = 2)]
        f32
    ),
}
fn main() {}
