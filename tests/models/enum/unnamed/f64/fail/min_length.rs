use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(min_length = 2)]
        f64
    ),
}
fn main() {}
