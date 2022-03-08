use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        i64
    ),
}
fn main() {}
