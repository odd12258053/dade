use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2.0)]
    value: u32,
}
fn main() {}
