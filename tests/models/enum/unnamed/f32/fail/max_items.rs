use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(max_items = 2)]
        f32
    ),
}
fn main() {}
