use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: u32,
}
fn main() {}
