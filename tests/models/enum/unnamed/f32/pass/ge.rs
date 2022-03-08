use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2.0)]
        f32
    ),
}
fn main() {}
