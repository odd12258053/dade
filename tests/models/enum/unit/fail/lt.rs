use dade::model;
#[model]
enum TestModel {
    #[field(lt = 2)]
    Value
}
fn main() {}
