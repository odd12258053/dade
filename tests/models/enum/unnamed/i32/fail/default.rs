use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1.0)]
        i32
    ),
}
fn main() {}
