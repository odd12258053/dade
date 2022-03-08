use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        u8
    ),
}
fn main() {}
