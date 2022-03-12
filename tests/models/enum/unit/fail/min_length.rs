use dade::model;
#[model]
enum TestModel {
    #[field(min_length = 2)]
    Value
}
fn main() {}
