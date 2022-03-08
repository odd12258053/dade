use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        u128
    ),
}
fn main() {}
