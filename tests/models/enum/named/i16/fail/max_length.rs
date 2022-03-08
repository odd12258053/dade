use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(max_length = 2)]
        i16
    ),
}
fn main() {}
