use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(default = 1.0)]
        u32
    ),
}
fn main() {}
