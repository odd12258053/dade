use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(max_length = 2)]
        f32
    ),
}
fn main() {}
