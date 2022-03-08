use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(max_length = 2)]
        u64
    ),
}
fn main() {}
