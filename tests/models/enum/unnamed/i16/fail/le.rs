use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2.0)]
        i16
    ),
}
fn main() {}
