use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2)]
        u32
    ),
}
fn main() {}
