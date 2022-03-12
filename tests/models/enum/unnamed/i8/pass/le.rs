use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2)]
        i8
    ),
}
fn main() {}
