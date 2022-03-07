use dade::model;
#[model]
struct TestModel {
    #[field(lt = 2.0)]
    value: f64,
}
fn main() {}
