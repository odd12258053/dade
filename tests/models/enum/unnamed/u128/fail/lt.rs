use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2.0)]
        u128
    ),
}
fn main() {}
