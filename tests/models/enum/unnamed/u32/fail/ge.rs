use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2.0)]
        u32
    ),
}
fn main() {}
