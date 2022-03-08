use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2.0)]
        i64
    ),
}
fn main() {}
