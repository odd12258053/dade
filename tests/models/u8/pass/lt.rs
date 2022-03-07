use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: u8,
}
fn main() {}
