use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        Vec<()>
    ),
}
fn main() {}
