use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2)]
        i128
    ),
}
fn main() {}
