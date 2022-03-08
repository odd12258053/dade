use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(max_length = 2)]
        f64
    ),
}
fn main() {}
