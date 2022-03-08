use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(alias = "val")]
        isize
    ),
}
fn main() {}
