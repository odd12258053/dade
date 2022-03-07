use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: isize,
}
fn main() {}
