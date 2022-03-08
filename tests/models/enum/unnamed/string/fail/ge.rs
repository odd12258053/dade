use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        String
    ),
}
fn main() {}
