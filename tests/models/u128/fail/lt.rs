use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2.0)]
    value: u128,
}
fn main() {}
