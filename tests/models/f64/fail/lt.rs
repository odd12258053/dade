use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2)]
    value: f64,
}
fn main() {}
