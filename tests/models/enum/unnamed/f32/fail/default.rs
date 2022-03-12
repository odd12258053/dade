use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1)]
        f32
    ),
}
fn main() {}
