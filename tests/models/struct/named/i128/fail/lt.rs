use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2.0)]
    value: i128,
}
fn main() {}
