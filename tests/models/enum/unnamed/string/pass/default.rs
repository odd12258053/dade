use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = "default")]
        String
    ),
}
fn main() {}
