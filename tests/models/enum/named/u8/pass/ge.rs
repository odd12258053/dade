use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        u8
    ),
}
fn main() {}
