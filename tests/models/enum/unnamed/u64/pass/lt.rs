use dade::model;
#[model]
enum TestModel {
    Value(
       #[field(lt = 2)]
        u64
    ),
}
fn main() {}
