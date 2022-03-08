use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2)]
        u128
    ),
}
fn main() {}
