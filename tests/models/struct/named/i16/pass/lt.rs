use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: i16,
}
fn main() {}
