use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2)]
        i32
    ),
}
fn main() {}
