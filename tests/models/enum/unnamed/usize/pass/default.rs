use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1)]
        usize
    ),
}
fn main() {}
