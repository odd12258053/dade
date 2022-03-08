use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(ge = 2)]
        u32
    ),
}
fn main() {}
