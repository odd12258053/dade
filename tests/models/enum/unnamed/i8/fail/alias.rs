use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        i8
    ),
}
fn main() {}
