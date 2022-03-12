use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = false)]
        bool
    ),
}
fn main() {}
