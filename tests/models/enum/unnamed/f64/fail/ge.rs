use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        f64
    ),
}
fn main() {}
