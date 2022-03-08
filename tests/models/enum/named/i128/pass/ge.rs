use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        i128
    ),
}
fn main() {}
