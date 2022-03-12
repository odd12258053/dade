use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1)]
        i8
    ),
}
fn main() {}
