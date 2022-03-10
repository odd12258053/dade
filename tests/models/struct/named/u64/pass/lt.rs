use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: u64,
}
fn main() {}
