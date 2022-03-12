use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2)]
        f32
    ),
}
fn main() {}
