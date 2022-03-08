use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2.0)]
        i64
    ),
}
fn main() {}
