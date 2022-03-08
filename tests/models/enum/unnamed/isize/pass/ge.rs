use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        isize
    ),
}
fn main() {}
