use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2)]
        isize
    ),
}
fn main() {}
