use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        u32
    ),
}
fn main() {}
