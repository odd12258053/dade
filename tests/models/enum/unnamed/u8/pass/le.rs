use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(le = 2)]
        u8
    ),
}
fn main() {}
