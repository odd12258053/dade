use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1.0)]
        i16
    ),
}
fn main() {}
