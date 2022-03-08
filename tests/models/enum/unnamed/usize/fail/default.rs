use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1.0)]
        usize
    ),
}
fn main() {}
