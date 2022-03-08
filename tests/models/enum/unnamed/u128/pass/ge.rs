use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        u128
    ),
}
fn main() {}
