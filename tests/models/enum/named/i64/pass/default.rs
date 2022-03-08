use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1)]
        i64
    ),
}
fn main() {}
