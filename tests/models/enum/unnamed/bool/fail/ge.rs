use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        bool
    ),
}
fn main() {}
