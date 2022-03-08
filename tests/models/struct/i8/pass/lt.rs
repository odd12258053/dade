use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: i8,
}
fn main() {}
