use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_length = 2)]
        i128
    ),
}
fn main() {}
