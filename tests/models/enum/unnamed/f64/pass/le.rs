use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2.0)]
        f64
    ),
}
fn main() {}
