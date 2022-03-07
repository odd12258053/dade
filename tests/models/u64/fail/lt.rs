use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2.0)]
    value: u64,
}
fn main() {}
