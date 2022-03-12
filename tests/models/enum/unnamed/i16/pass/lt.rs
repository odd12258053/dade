use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2)]
        i16
    ),
}
fn main() {}
