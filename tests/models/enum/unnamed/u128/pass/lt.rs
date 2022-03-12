use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2)]
        u128
    ),
}
fn main() {}
