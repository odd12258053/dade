use dade::model;
#[model]
enum TestModel {
    Value{
       #[field(min_length = 2)]
        value: i8
    },
}
fn main() {}
