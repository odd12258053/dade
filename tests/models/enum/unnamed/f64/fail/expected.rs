use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(expected = "val")]
        f64
    ),
}
fn main() {}
