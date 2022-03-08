use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: u16,
}
fn main() {}
