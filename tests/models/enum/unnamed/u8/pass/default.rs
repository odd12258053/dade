use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1)]
        u8
    ),
}
fn main() {}
