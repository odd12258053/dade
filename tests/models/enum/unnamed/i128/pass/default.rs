use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1)]
        i128
    ),
}
fn main() {}
