use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2.0)]
        i32
    ),
}
fn main() {}
