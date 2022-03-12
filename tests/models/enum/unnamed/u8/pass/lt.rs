use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2)]
        u8
    ),
}
fn main() {}
