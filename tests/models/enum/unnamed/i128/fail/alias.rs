use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        i128
    ),
}
fn main() {}
