use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        u16
    ),
}
fn main() {}
