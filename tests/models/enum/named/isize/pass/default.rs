use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1)]
        isize
    ),
}
fn main() {}
