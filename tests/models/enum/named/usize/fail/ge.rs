use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2.0)]
        usize
    ),
}
fn main() {}
