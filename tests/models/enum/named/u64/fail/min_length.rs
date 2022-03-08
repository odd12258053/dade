use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_length = 2)]
        u64
    ),
}
fn main() {}
