use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        bool
    ),
}
fn main() {}
