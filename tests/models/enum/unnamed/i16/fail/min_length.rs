use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_length = 2)]
        i16
    ),
}
fn main() {}
