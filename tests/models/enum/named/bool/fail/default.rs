use dade::model;
#[model]
enum TestModel {
    Value{
       #[field(default = 1)]
        value: bool
    },
}
fn main() {}
