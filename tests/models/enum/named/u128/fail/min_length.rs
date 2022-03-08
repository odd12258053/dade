use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_length = 2)]
        u128
    ),
}
fn main() {}
