use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        i16
    ),
}
fn main() {}
