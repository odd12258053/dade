use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        String
    ),
}
fn main() {}
