use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2)]
        u16
    ),
}
fn main() {}
