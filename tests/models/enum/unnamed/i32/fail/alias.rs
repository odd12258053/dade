use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        i32
    ),
}
fn main() {}
