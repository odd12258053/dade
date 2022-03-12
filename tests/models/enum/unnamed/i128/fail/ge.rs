use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2.0)]
        i128
    ),
}
fn main() {}
