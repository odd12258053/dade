use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        u64
    ),
}
fn main() {}
