use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: i32,
}
fn main() {}
