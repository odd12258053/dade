use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        f32
    ),
}
fn main() {}
