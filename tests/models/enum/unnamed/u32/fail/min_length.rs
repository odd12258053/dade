use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_length = 2)]
        u32
    ),
}
fn main() {}
