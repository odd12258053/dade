use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2.0)]
        i32
    ),
}
fn main() {}
