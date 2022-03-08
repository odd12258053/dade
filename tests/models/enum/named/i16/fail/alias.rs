use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        i16
    ),
}
fn main() {}
