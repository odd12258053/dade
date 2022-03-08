use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        u16
    ),
}
fn main() {}
