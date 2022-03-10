use dade::model;
#[model]
enum TestModel {
    #[field(max_length = 2)]
    Value
}
fn main() {}
